use crate::usecase::interface::TrelloApiGateway;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

fn is_valid_prefix_word(prefix_word: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z\-#]+$").unwrap();
    return re.is_match(&prefix_word);
}

fn search_trello_card_numbers(
    prefix_word: String,
    vals: Vec<String>,
) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut tcns: HashSet<String> = HashSet::new();
    let re = format!("{}([0-9]+)", prefix_word);
    let search_re = Regex::new(&re).unwrap();
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
        prefix_word: String,
        gh_repository_name: String,
        gh_branch_name: String,
    ) -> Result<(), Box<dyn Error>> {
        if !is_valid_prefix_word(prefix_word.to_string()) {
            println!("invalid prefix word");
            return Ok(());
        }
        let gh_branch_url = create_branch_url(gh_repository_name, gh_branch_name.to_string());
        let tcns =
            search_trello_card_numbers(prefix_word, vec![gh_branch_name.to_string()]).unwrap();
        if tcns.is_empty() {
            println!("not found trello card numbers.");
            return Ok(());
        }
        for tcn in tcns {
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
            let mut target_urls = HashSet::from([gh_branch_url.to_string()]);
            for a in attachments {
                if a.url == gh_branch_url && target_urls.contains(&gh_branch_url) {
                    // don't attach if already attach link
                    target_urls.remove(&gh_branch_url);
                    break;
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
