use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env;
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    //optional
    user_store: LookupMap<String, String>,
}

//optional, not needed for challenge
// this is to show you how to setup a lookup map to store the last
// string an user entered.
impl Default for HelloWorld {
    fn default() -> Self {
        Self {
            user_store: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl HelloWorld {
    pub fn set_name(&mut self, name: String) {
        //optional if you wanted to learn a little about collections
        // this is a key store example using a lookup map
        let account_name = env::signer_account_id();
        self.user_store.insert(&account_name, &name);

        //For the challenge this is all you need
        let print_message = format!("Hello {}!", name);
        env::log(print_message.as_bytes())
    }
}
