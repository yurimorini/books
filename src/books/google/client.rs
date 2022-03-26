use futures::future::join_all;

use super::get_volume;
use super::search_isbn;
use super::{Isbn, Volume};

#[derive(Debug)]
pub struct ApiConfig {
    pub base_uri: String,
    pub api_key: String,
}

#[derive(Debug)]
pub struct Client {
    config: ApiConfig,
}

impl Client {
    pub fn new(config: ApiConfig) -> Client {
        Client { config }
    }

    pub async fn search_book(&self, isbn: &Isbn) -> Option<Volume> {
        match search_isbn(&isbn.to_string(), &self.config).await {
            Some(id) => get_volume(&id, &self.config).await.map(|mut volume| {
                volume.isbn = isbn.clone();
                volume
            }),
            None => None,
        }
    }

    pub async fn search_books(&self, list: Vec<Isbn>) -> Vec<Volume> {
        join_all(list.iter().map(|isbn| self.search_book(isbn)))
            .await
            .into_iter()
            .flatten()
            .collect()
    }
}
