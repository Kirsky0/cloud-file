use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::Serialize;


use crate::user::User;

use crate::canister::create;
use crate::canister::install;
use std::mem;
use base64;

lazy_static! {
     static ref USER_MAP: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
     static ref wasm_str: Mutex<String> = Mutex::new(String::default());
}


#[update(name = "add_user")]
pub async fn add_user(username: String) -> String
{
    let canister = create::create_canister().await;
    let u = User::new(canister, username);

    let canister_str = canister.to_text();
    USER_MAP.lock().unwrap().insert(canister_str.clone(), u);
    return canister_str;
    // userMap.insert(user_main_canister.to_text(), u);
}

#[update(name = "init_user")]
pub async fn init_user(canister_id: String)
{
    let byte = base64::decode(wasm_str.lock().unwrap().clone()).unwrap();
    let p = Principal::from_text(canister_id).unwrap();
    install::install_code(p, byte).await;
}

#[query(name = "show_user")]
pub async fn show_user(canister_id: String) -> String
{
    let map = USER_MAP.lock().unwrap();
    let username = map.get(&canister_id).unwrap().get_username().clone();
    return username;
}

#[query(name = "mem_size")]
pub fn mem_size() -> u64 {
    return super::mem::get_heap_memory_size();
    // return std::mem::size_of::<u64>() as u64;
}

#[update(name = "set_wasm")]
pub async fn input_wasm(mut str: String)
{
    // let header = String::from("data:application/wasm;base64,");

    let data_body = str.split_off(29);
    let mut setStr = wasm_str.lock().unwrap();
    *setStr = data_body;
}

#[query(name = "get_wasm")]
pub fn get_wasm() -> String {
    return wasm_str.lock().unwrap().clone();
}