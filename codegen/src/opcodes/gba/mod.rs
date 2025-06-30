use super::raw::RawOpcode;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Root {
    pub arm: HashMap<String, RawOpcode>,
    pub thumb: HashMap<String, RawOpcode>,
}
