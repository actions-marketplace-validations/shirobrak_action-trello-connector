use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

use crate::usecase::interface::TrelloApiGateway;

fn search_trello_card_numbers(vals: Vec<String>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut tcns: HashSet<String> = HashSet::new();
    let search_re = Regex::new(r"#tcn([0-9]+)").unwrap();
    for val in vals {
        for cap in search_re.captures_iter(&val) {
            tcns.insert(String::from(&cap[1]));
        }
    }
    return Ok(tcns);
}

fn create_branch_url(repository_name: String, branch_name: String) -> String {
    return format!("https://github.com/{repository_name}/tree/{branch_name}");
}

pub struct Usecase {
    trello_api_gw: Box<dyn TrelloApiGateway>,
}

impl Usecase {
    pub fn new(trello_api_gw: Box<dyn TrelloApiGateway>) -> Usecase {
        return Usecase { trello_api_gw };
    }

    pub fn run(
        &self,
        board_id: String,
        gh_repository_name: String,
        gh_pr_url: String,
        gh_branch_name: String,
        gh_pr_title: String,
        gh_pr_body: String,
    ) -> Result<(), Box<dyn Error>> {
        let gh_branch_url = create_branch_url(gh_repository_name, gh_branch_name.to_string());
        let tcns = search_trello_card_numbers(vec![
            gh_pr_title.to_string(),
            gh_pr_body.to_string(),
            gh_branch_name.to_string(),
        ])
        .unwrap();
        if tcns.is_empty() {
            println!("not found trello card numbers.");
            return Ok(());
        }
        for tcn in tcns {
            // カードを検索する
            let res = self
                .trello_api_gw
                .find_card_by_card_num(board_id.to_string(), tcn.to_string());
            if res.is_err() {
                return Err(res.err().unwrap());
            }
            let card_with_option = res.ok().unwrap();
            if card_with_option.is_none() {
                println!("not found card.");
                continue;
            }
            let card = card_with_option.unwrap();
            let res = self.trello_api_gw.fetch_attachments_on_card(card.clone());
            if res.is_err() {
                return Err(res.err().unwrap());
            }
            let attachments = res.ok().unwrap();
            let mut target_urls = HashSet::from([gh_pr_url.to_string(), gh_branch_url.to_string()]);
            for a in attachments {
                // don't attach if already attach link
                if a.url == gh_pr_url && target_urls.contains(&gh_pr_url) {
                    target_urls.remove(&gh_pr_url);
                }
                if a.url == gh_branch_url && target_urls.contains(&gh_branch_url) {
                    target_urls.remove(&gh_branch_url);
                }
            }

            for target_url in target_urls {
                let res = self.trello_api_gw.post_attachment(
                    card.clone(),
                    "attachment post by GitHub Action".to_string(),
                    target_url,
                );
                if res.is_err() {
                    return Err(res.err().unwrap());
                }
            }
        }
        Ok(())
    }
}
