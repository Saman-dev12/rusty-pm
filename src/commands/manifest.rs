use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct RustyManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,  
    pub scripts: std::collections::HashMap<String, String>,
}


