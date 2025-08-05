use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct RustyManifest {
    name: String,
    version: String,
    author: String,
    description: String,
  dependencies: Vec<String>,  
}


