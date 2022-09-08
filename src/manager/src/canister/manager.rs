use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;
use candid::error::Error;
use candid::{Encode};
use ic_cdk::export::Principal;
use ic_cdk::{api, id};
use ic_cdk::api::call::CallResult;
use once_cell::sync::Lazy;

use crate::canister::types::{CanisterInstall, CanisterSettings, CreateResult, InstallMode, StoreUnit, InstallArgs};

pub static STORE_UNIT_MAP: Lazy<Mutex<HashMap<String, StoreUnit>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub async fn add_canister(cycles: u128, wasm_byte: Vec<u8>) -> String
{
    // let cycles: u128 = 800_000_000_000;

    let canister_id = create_canister(cycles).await;
    // let canister_id = create_canister(cycles).await;

    // let store_unit_new = StoreUnit {
    //     canister_id: canister_id,
    //     controllers: vec![id()],
    //     size: 0,
    //     cycle: 0,
    //     installed_code: false,
    // };
    // STORE_UNIT_MAP.lock().unwrap().insert(canister_id.clone().to_text(), store_unit_new);

    let success = install_canister(canister_id, wasm_byte).await;
    if success
    {
        return canister_id.to_text();
        // let mut map = STORE_UNIT_MAP.lock().unwrap();
        // let mut store_unit = map.get_mut(&canister_id.to_text()).unwrap();
        // store_unit.installed_code = true;
    };
    return "err".to_string();
}

async fn create_canister(cycles: u128) -> Principal {
    let setting = CanisterSettings {
        controllers: vec![ic_cdk::api::caller()],
    };
    // let r: CallResult<(CreateResult, )> = api::call::call_with_payment128(
    //     Principal::management_canister(),
    //     "create_canister",
    //     (setting, ),
    //     cycles,
    // ).await;
    // if let Err((code, msg)) = r {
    //     ic_cdk::api::trap(&msg);
    // }
    // return r.unwrap().0.canister_id;

    let (create_result, ): (CreateResult, ) = match api::call::call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (setting, ),
        cycles,
    ).await {
        Ok(o) => o,
        Err((code, msg)) => {
            ic_cdk::api::trap(&msg);
        }
    };
    return create_result.canister_id;
}


async fn install_canister(canister_id: Principal, wasm_byte: Vec<u8>) -> bool
{
    let args = InstallArgs {
        controller: id().to_text(),
    };
    let arg_encode = Encode!(&args).unwrap();
    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id: canister_id,
        wasm_module: wasm_byte,
        arg: arg_encode,
    };
    // let result: CallResult<((), )> = api::call::call(
    //     Principal::management_canister(),
    //     "install_code",
    //     (install_config, ),
    // ).await;
    // if let Err((code, msg)) = result {
    //     ic_cdk::api::trap(&msg);
    // }


    match api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config, ),
    ).await as CallResult<((), )> {
        Ok(_) => {}
        Err((code, msg)) => {
            ic_cdk::api::trap(&msg);
        }
    };
    return true;
}
