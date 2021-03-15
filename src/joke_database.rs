use std::time::{SystemTime, UNIX_EPOCH};

use mongodb::{bson::{doc, Bson}, options::AggregateOptions, sync::{Client, Collection}};

pub struct Joke_Database{
    server_name: String,
    username: String,
    collection: mongodb::sync::Collection
}

impl Joke_Database {
    pub fn new() -> Result<Self, String>{
        let server_name = std::env::var("MONGO_SERVER");
        let username = std::env::var("MONGO_USERNAME");
        let password = std::env::var("MONGO_PASSWORD");
        let port = std::env::var("MONGO_PORT");

        match (server_name, username, password, port){
            (Ok(server_name), Ok(username), Ok(password), Ok(port)) => {
                let connection_string = format!("mongodb://{}:{}@{}:{}", 
                    username.clone(), 
                    password.clone(), 
                    server_name.clone(),
                    port.clone());
                
                let client = Client::with_uri_str(connection_string.as_str()).expect("Could not create client");
                let collection = client.database("joke_bot").collection("jokes");

                Ok(Joke_Database{
                    server_name: server_name,
                    username: username,
                    collection: collection
                })
            }
            _ => return Err("Could not get environment variables".to_string())
        }
    }

    pub fn get_random_joke(&self) -> Option<String>{
        let cursor = self.collection.find(None, None).expect("Could not get cursor");
        let size = cursor.count();

        let mut cursor = self.collection.find(None, None).expect("Could not get cursor");
        let selector = SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get time since epoch").as_millis() as usize;

        let mut joke = cursor.nth(selector % size ).expect("got them joke boi").expect("AAAH");
        let joke_string = joke.get("joke").and_then(Bson::as_str).expect("HAHAHAHa").to_string();
        return Some(joke_string);
    }

    /* Private methods */
}