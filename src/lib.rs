use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    name: String,
}

#[near_bindgen]
impl HelloWorld {
    pub fn set_name(&mut self, name: String) {
        self.name = format!("{} {}{}", "Hello", name, "!");
    }

    pub fn get_message(&self) -> String {
        let result = self.name.clone();
        result
    }
}
