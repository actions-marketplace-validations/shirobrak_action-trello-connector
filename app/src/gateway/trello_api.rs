use std::{collections::HashMap, error::Error};

use reqwest::{blocking::Client as _Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::usecase::interface::TrelloApiGateway;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub id: Option<String>,
    pub name: String,
    pub url: String,
}

pub struct Client {
    api_key: String,
    api_token: String,
    client: _Client,
}

impl Client {
    pub fn new(api_key: String, api_token: String) -> Client {
        let client = _Client::new();
        return Client {
            api_key,
            api_token,
            client,
        };
    }
}

impl TrelloApiGateway for Client {
    fn find_card_by_card_num(
        &self,
        board_id: String,
        card_num: String,
    ) -> Result<Option<Card>, Box<dyn Error>> {
        let url = format!(
            "https://api.trello.com/1/boards/{}/cards/{}?key={}&token={}",
            board_id, card_num, self.api_key, self.api_token
        );
        let res = self.client.get(&url).send()?;
        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let card: Card = res.json()?;
        return Ok(Some(card));
    }

    fn post_attachment(
        &self,
        card: Card,
        name: String,
        url: String,
    ) -> Result<Attachment, Box<dyn Error>> {
        let mut post_data = HashMap::new();
        post_data.insert("name", name);
        post_data.insert("url", url);
        let url = format!("https://api.trello.com/1/cards/{card_id}/attachments?key={trello_key}&token={trello_token}", card_id = card.id, trello_key = self.api_key, trello_token = self.api_token);
        let res = self.client.post(&url).json(&post_data).send()?;
        let created_attachment: Attachment = res.json()?;
        Ok(created_attachment)
    }

    fn fetch_attachments_on_card(&self, card: Card) -> Result<Vec<Attachment>, Box<dyn Error>> {
        let url = format!("https://api.trello.com/1/cards/{card_id}/attachments?key={trello_key}&token={trello_token}", card_id = card.id, trello_key = self.api_key, trello_token = self.api_token);
        let res = self.client.get(&url).send()?;
        let attachments: Vec<Attachment> = res.json()?;
        Ok(attachments)
    }
}
