use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use candid::candid_method;
use ic_cdk::{call, caller, id};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::*;
use ic_types::Principal;
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::canister::manager::{canister_status, create_canister, install_canister, update_setting};
use crate::canister::types::{CanisterStatus, StoreUnit};

// static wasm_byte: Lazy<Mutex<String>> = Lazy::new(|| {
//     Mutex::new(String::default())
// });
// static STORE_UNIT_MAP: Lazy<Mutex<HashMap<String, StoreUnit>>> = Lazy::new(|| {
//     Mutex::new(HashMap::new())
// });


#[query(name = "mem_size")]
#[candid_method(query)]
fn mem_size() -> u64 {
    return super::mem::get_heap_memory_size();
}

// #[update(name = "upload_wasm")]
// #[candid_method(update)]
// pub fn input_wasm(mut str: String)
// {
//     let data_body = str.split_off(29);
//     let mut setStr = wasm_byte.lock().unwrap();
//     *setStr = data_body;
// }
//
// #[query(name = "get_wasm")]
// #[candid_method(query)]
// pub fn get_wasm() -> String {
//     return wasm_byte.lock().unwrap().clone();
// }


#[update(name = "upload_module")]
#[candid_method(update)]
async fn upload_module(byte: Vec<u8>) -> String {
    let cycles: u128 = 800_000_000_000;
    // let id = add_canister(cycles, byte).await;
    let canister_id = create_canister(cycles).await;


    // let store_unit_new = StoreUnit {
    //     canister_id: canister_id,
    //     controllers: vec![id()],
    //     size: 0,
    //     cycle: 0,
    //     installed_code: false,
    // };
    // STORE_UNIT_MAP.lock().unwrap().insert(canister_id.clone().to_text(), store_unit_new);
    if Some(canister_id).is_some()
    {
        install_canister(&canister_id, byte).await;
    }


    // let mut map = STORE_UNIT_MAP.lock().unwrap();
    // let mut store_unit = map.get_mut(&canister_id.to_text()).unwrap();
    // store_unit.installed_code = true;

    // let controllers = vec![canister_id.clone(), caller(), id()];
    // update_setting(canister_id.clone(), controllers).await;

    return canister_id.to_text();
}

// #[query(name = "store_list")]
// #[candid_method(query)]
// pub fn store_list() -> String {
//     let store_unit = STORE_UNIT_MAP.lock().unwrap().clone();
//     let json_str = serde_json::to_string(&store_unit);
//     return json_str.unwrap();
// }

#[update(name = "store_status")]
#[candid_method(update)]
async fn store_status(id_str: String) -> CanisterStatus {
    let status = canister_status(Principal::from_text(id_str).unwrap()).await;
    return status;
}

