use clap::Parser;

use crate::Params;

#[derive(Parser, Debug)]
#[clap(
    name = "Action Trello Connector",
    author = "shirobrak",
    version = "v1.0.0",
    about = "This app is the cli for Trello and GitHub integration"
)]
struct Args {
    #[clap(long, value_parser)]
    trello_board_id: String,

    #[clap(long, value_parser)]
    gh_event_name: String,

    #[clap(long, value_parser)]
    gh_repository_name: String,

    #[clap(long, value_parser)]
    gh_pr_num: Option<String>,

    #[clap(long, value_parser)]
    gh_pr_url: Option<String>,

    #[clap(long, value_parser)]
    gh_pr_title: Option<String>,

    #[clap(long, value_parser)]
    gh_pr_body: Option<String>,

    #[clap(long, value_parser)]
    gh_pr_branch_name: Option<String>,

    #[clap(long, value_parser)]
    gh_push_branch_name: Option<String>,
}

pub struct CLI {}

impl CLI {
    pub fn parse_to_params() -> Params {
        let args = Args::parse();
        return Params {
            t_board_id: args.trello_board_id,
            gh_event_name: args.gh_event_name,
            gh_repository_name: args.gh_repository_name,
            gh_pr_num: args.gh_pr_num,
            gh_pr_url: args.gh_pr_url,
            gh_pr_title: args.gh_pr_title,
            gh_pr_body: args.gh_pr_body,
            gh_pr_branch_name: args.gh_pr_branch_name,
            gh_push_branch_name: args.gh_push_branch_name,
        };
    }
}
