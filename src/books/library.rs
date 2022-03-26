use super::{google::Client, Isbn, Storage, Volume};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

/// Storage for the downloaded Volumes list
///
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    pub volumes: Vec<Volume>,
}

/// High level service to manage the library structure.
///
/// It implements service api and load save the library from the disk
/// abstracting the client to search book and the storage engine.
///
/// Once created call immediately the storage loading the library.
///
pub struct LibraryService {
    client: Rc<Client>,
    storage: Rc<Storage>,
    library: Library,
}

pub struct AppendStats {
    pub input_list: usize,
    pub new_volumes: usize,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl LibraryService {
    /// Instantiate a new service. Loads the library immediately.
    ///
    pub fn new(client: Rc<Client>, storage: Rc<Storage>) -> LibraryService {
        let library = storage.load();
        LibraryService {
            library,
            client,
            storage,
        }
    }

    /// Search new volumes and append them to the library.
    ///
    /// Uses the client to search new books, but before search
    /// based on the ISBN filter the input list skipping ISBNs
    /// already imported in the library.
    ///
    /// Once found, mutate the library and append the new books.
    ///
    pub async fn append_volumes(&mut self, list: &[Isbn]) -> AppendStats {
        let filtered = self.identify_new_isbns(list);
        let new_volumes: Vec<Volume> = self.client.search_books(filtered).await;

        let stats = AppendStats {
            input_list: list.len(),
            new_volumes: new_volumes.len(),
        };

        for volume in new_volumes.into_iter() {
            self.library.volumes.push(volume);
        }

        stats
    }

    /// Save the library to the disk.
    ///
    pub fn save(&self) -> Result<(), &str> {
        self.storage
            .save(&self.library)
            .map_err(|_| "Impossible to write library")
    }

    /// Filters ISBNs comparing every input ISBN
    /// with all the ISBNs already loaded in the library.
    ///
    fn identify_new_isbns(&self, isbns: &[Isbn]) -> Vec<Isbn> {
        let mut list = vec![];
        list.extend_from_slice(isbns);
        for volume in self.library.volumes.iter() {
            let isbn = &volume.isbn;
            if let Some(pos) = list.iter().position(|x| x == isbn) {
                list.remove(pos);
            };
        }
        list
    }
}
