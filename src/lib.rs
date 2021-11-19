use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    hello_message: LookupMap<String, String>,
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self {
            hello_message: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl HelloWorld {
    pub fn set_name(&mut self, name: String) {
        self.hello_message
            .insert(&name, &format!("{} {}{}", "Hello", name, "!"));
        let print_message = format!("Hello {}", name);
        env::log(print_message.as_bytes())
    }

    pub fn get_message(&self, name: String) -> String {
        match self.hello_message.get(&name) {
            Some(message) => message,
            None => "No message found".to_string(),
        }
    }
}
