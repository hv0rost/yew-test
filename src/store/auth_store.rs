use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local")]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authentificated : bool,
}
