use std::any::Any;

pub trait Module: Any + Send + Sync {
    fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>>;
}