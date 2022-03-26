use serde::{Deserialize, Serialize};

/// Book Isbn
///
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Isbn {
    pub value: String,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl Isbn {
    pub fn new(raw: &str) -> Isbn {
        let value = str::replace(raw, "-", "");
        Isbn { value }
    }
}

impl ToString for Isbn {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
