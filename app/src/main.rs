mod cli;
mod gateway;
mod usecase;

use cli::CLI;
use gateway::github_api::Client as GitHubApiClient;
use gateway::trello_api::Client as TrelloApiClient;

#[derive(Debug)]
struct Envs {
    t_key: String,
    t_token: String,
    gh_token: String,
}

#[derive(Debug)]
pub struct Params {
    t_board_id: String,
    gh_event_name: String,
    gh_repository_name: String,
    gh_pr_num: Option<String>,
    gh_pr_url: Option<String>,
    gh_pr_title: Option<String>,
    gh_pr_body: Option<String>,
    gh_pr_branch_name: Option<String>,
    gh_push_branch_name: Option<String>,
}

fn get_envs() -> Envs {
    let t_key = std::env::var("TRELLO_KEY").expect("`TRELLO_KEY` is not set.");
    let t_token = std::env::var("TRELLO_TOKEN").expect("`TRELLO_TOKEN` is not set.");
    let gh_token = std::env::var("GITHUB_TOKEN").expect("`GITHUB_TOKEN` is not set.");
    return Envs {
        t_key,
        t_token,
        gh_token,
    };
}

fn main() {
    let envs = get_envs();
    let params = CLI::parse_to_params();
    if params.gh_event_name == "pull_request" {
        let t_api_client = TrelloApiClient::new(envs.t_key, envs.t_token);
        let gh_api_client = GitHubApiClient::new(envs.gh_token);
        let usecase =
            usecase::on_pr_usecase::Usecase::new(Box::new(t_api_client), Box::new(gh_api_client));
        let gh_pr_num = params
            .gh_pr_num
            .expect("`gh_pr_num` is required param, when pr event");
        let gh_pr_url = params
            .gh_pr_url
            .expect("`gh_pr_url` is required param, when pr event");
        let gh_branch_name = params
            .gh_pr_branch_name
            .expect("`gh_pr_branch_name` is required param, when pr event");
        let gh_pr_title = params
            .gh_pr_title
            .expect("`gh_pr_title` is required param, when pr event");
        let gh_pr_body = params
            .gh_pr_body
            .expect("`gh_pr_body` is required param, when pr event");
        let _ = usecase.run(
            params.t_board_id,
            params.gh_repository_name,
            gh_pr_num,
            gh_pr_url,
            gh_branch_name,
            gh_pr_title,
            gh_pr_body,
        );
    } else if params.gh_event_name == "push" {
        let t_api_client = TrelloApiClient::new(envs.t_key, envs.t_token);
        let usecase = usecase::on_push_usecase::Usecase::new(Box::new(t_api_client));
        let gh_branch_name = params
            .gh_push_branch_name
            .expect("`gh_pr_branch_name` is required param, when pr event");
        let _ = usecase.run(params.t_board_id, params.gh_repository_name, gh_branch_name);
    }
}
