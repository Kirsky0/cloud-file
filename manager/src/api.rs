use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use base64;
use candid::candid_method;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::canister::manager::add_canister;
use crate::canister::manager::STORE_UNIT_MAP;

static wasm_byte: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::default())
});


#[query(name = "mem_size")]
#[candid_method(query)]
pub fn mem_size() -> u64 {
    return super::mem::get_heap_memory_size();
}

#[update(name = "upload_wasm")]
#[candid_method(update)]
pub fn input_wasm(mut str: String)
{
    // let header = String::from("data:application/wasm;base64,");

    let data_body = str.split_off(29);
    let mut setStr = wasm_byte.lock().unwrap();
    *setStr = data_body;
}


#[query(name = "get_wasm")]
#[candid_method(query)]
pub fn get_wasm() -> String {
    return wasm_byte.lock().unwrap().clone();
}

#[update(name = "create_module")]
#[candid_method(update)]
pub async fn create_module(mut str: String) -> String {
    let data_body = str.split_off(29);
    let wasm_bytes = base64::decode(data_body).unwrap();

    let cycles: u128 = 800_000_000_000;
    let id = add_canister(cycles, wasm_bytes).await;

    return id;
}

#[query(name = "get_store")]
#[candid_method(query)]
pub fn get_store() -> String {
    let store_unit = STORE_UNIT_MAP.lock().unwrap().clone();
    let json_str = serde_json::to_string(&store_unit);
    return json_str.unwrap();
}