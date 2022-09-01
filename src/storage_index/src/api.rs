use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use ic_cdk::export::Principal;
use ic_cdk_macros::*;

lazy_static! {
     static ref KEY_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


#[update(name = "add_data_key")]
pub async fn add_record(key: String, data_canister: String)
{
    KEY_MAP.lock().unwrap().insert(key, data_canister);
}

#[update(name = "remove_data_key")]
pub async fn remove_record(key: String)
{
    KEY_MAP.lock().unwrap().remove(&key);
}