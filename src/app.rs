use crate::books::google::{ApiConfig, Client};
use crate::books::{Isbn, LibraryService, Storage};
use crate::config::Config;
use std::rc::Rc;

pub use crate::books::AppendStats as Stats;

pub struct FetchCommand {
    service: LibraryService,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl FetchCommand {
    // Creates the services dependencies using the Config provided
    //
    // The config usually is used to define the auth data for the client
    // and the storage folder where to save output library data
    //
    pub fn create(config: &Config) -> FetchCommand {
        let client = Rc::new(create_client(config));
        let storage = Rc::new(create_storage(config));
        let service = LibraryService::new(client.clone(), storage.clone());

        FetchCommand { service }
    }

    // Fetches new volumes based on the input list.
    //
    // It returns a stat object with the input data count and the volume
    // appended. These values could be different (input is filtered).
    //
    pub async fn run(&mut self, list: &[Isbn]) -> Result<Stats, &str> {
        let stat = self.service.append_volumes(list).await;
        self.service.save()?;

        Ok(stat)
    }
}

// Extract data from config to create the client
//
// Pay attention to the fact the Config is considered
// invalid if these data is not set so we should have
// both if this function is called.
//
fn create_client(config: &Config) -> Client {
    let api_config = ApiConfig {
        base_uri: config.base_url.clone(),
        api_key: config.api_key.clone(),
    };
    Client::new(api_config)
}

// Create the storage service passing where to put
// the storage from Config.
//
fn create_storage(config: &Config) -> Storage {
    Storage::new(&config.output)
}
