use crate::gateway::trello_api::{Attachment, Card};
use std::error::Error;

pub trait TrelloApiGateway {
    fn find_card_by_card_num(
        &self,
        board_id: String,
        card_num: String,
    ) -> Result<Option<Card>, Box<dyn Error>>;
    fn post_attachment(
        &self,
        card: Card,
        name: String,
        url: String,
    ) -> Result<Attachment, Box<dyn Error>>;
    fn fetch_attachments_on_card(&self, card: Card) -> Result<Vec<Attachment>, Box<dyn Error>>;
}
