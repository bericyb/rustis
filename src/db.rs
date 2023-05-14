use std::fs;
use std::sync::Arc;

use dashmap::{mapref::one::Ref, DashMap};
use serde_json::Value;

pub struct Database {
    data: Arc<DashMap<String, Value>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(DashMap::new()),
        }
    }

    pub async fn get(&self, key: &String) -> Option<Ref<String, Value>> {
        return self.data.get(key);
    }

    pub async fn insert(&self, key: String, value: Value) -> Option<Value> {
        return self.data.insert(key, value);
    }

    pub async fn update(&self, key: String, value: Value) -> Option<Ref<String, Value>> {
        self.data.alter(&key, |_, _| value);
        self.data.get(&key)
    }

    pub async fn delete(&self, key: String) -> Option<(String, Value)> {
        self.data.remove(&key)
    }

    pub async fn read_from_file(&self, filename: &str) {
        let data_string = match fs::read_to_string(filename) {
            Ok(data) => data,
            Err(_err) => {
                println!(
                    "Unable to read from {} defaulting to empty database",
                    filename
                );
                return;
            }
        };

        if data_string.len() < 1 {
            return;
        }

        println!("Found existing data, loading now...");

        let json_data: Vec<(String, Value)> = match serde_json::from_str(&data_string) {
            Ok(data) => data,
            Err(err) => {
                println!("Unable to parse json correctly : {}", err);
                return;
            }
        };

        for entry in json_data.clone().into_iter() {
            self.data.insert(entry.0, entry.1);
        }

        println!("Loaded {} entries", json_data.len());
    }

    pub async fn write_to_file(&self, filename: &str) {
        let hashmap = self
            .data
            .iter()
            .map(|entry| (entry.key().to_string(), entry.value().clone()))
            .collect::<Vec<(String, Value)>>();

        let json = serde_json::to_string(&hashmap).unwrap();

        fs::write(filename, json).expect(&format!("Unable to write to {}", filename));

        println!("Finished writing {} entries to {}", hashmap.len(), filename)
    }
}
