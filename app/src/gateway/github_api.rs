use std::collections::HashMap;
use std::error::Error;

use reqwest::{
    blocking::Client as _Client,
    header::{HeaderMap, AUTHORIZATION, USER_AGENT},
    StatusCode,
};

use crate::usecase::interface::GitHubApiGateway;

pub struct Client {
    client: _Client,
    github_token: String,
}

impl Client {
    pub fn new(github_token: String) -> Client {
        let client = _Client::new();
        return Client {
            client,
            github_token,
        };
    }
}

impl GitHubApiGateway for Client {
    fn attach_trello_link_to_pr(
        &self,
        repository_name: String,
        pr_num: String,
        card_url: String,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.github.com/repos/{repository_name}/issues/{pr_num}/comments",
            repository_name = repository_name.to_string(),
            pr_num = pr_num.to_string()
        );
        let mut post_data: HashMap<&str, String> = HashMap::new();
        post_data.insert("body", card_url);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("token {}", self.github_token).parse().unwrap(),
        );
        headers.insert(USER_AGENT, "action-trello-connector".parse().unwrap());
        let res = self
            .client
            .post(&url)
            .headers(headers)
            .json(&post_data)
            .send()?;
        if res.status() != StatusCode::CREATED {
            println!("failed to create comment")
        }
        Ok(())
    }
}
