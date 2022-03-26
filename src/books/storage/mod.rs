use super::Library;
use std::fs::File;
use std::io::{BufReader, Error};

/// Storage service, load and save the library to disk.
///
/// It serialize and deserialize to the internal structure.
///
pub struct Storage {
    path: String,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl Storage {
    pub fn new(path: &str) -> Storage {
        Storage {
            path: path.to_owned(),
        }
    }

    pub fn load(&self) -> Library {
        File::open(&self.path).map_or(Library::default(), |file| {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        })
    }

    pub fn save(&self, library: &Library) -> Result<(), Error> {
        File::create(&self.path)
            .and_then(|file| serde_json::to_writer_pretty(file, library).map_err(Error::from))
    }
}
