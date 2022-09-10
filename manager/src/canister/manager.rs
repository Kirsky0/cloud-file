use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;
use candid::error::Error;
use candid::{Encode};
use ic_cdk::{api, call};
use ic_cdk::api::{caller, id};
use ic_cdk::api::call::CallResult;
use ic_types::Principal;
use once_cell::sync::Lazy;

use crate::canister::types::{CanisterInstall, CanisterSettings, CreateResult, InstallMode, StoreUnit, InstallArgs, CreateCanisterArgs, StatusArgs, CanisterStatus, UpdateSettings};


pub async fn create_canister(cycles: u128) -> Principal {
    let settings = CanisterSettings {
        controllers: Some(vec![caller(), id()]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };
    let args = CreateCanisterArgs {
        settings: Some(settings)
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
        (args, ),
        cycles,
    ).await {
        Ok(o) => o,
        Err((code, msg)) => {
            ic_cdk::api::trap(&msg);
        }
    };
    return create_result.canister_id;
}


pub async fn install_canister(canister_id: &Principal, wasm_byte: Vec<u8>)
{
    let args = InstallArgs {
        controller: id().to_text(),
    };
    let arg_encode = Encode!(&args).unwrap();

    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id:  *canister_id,
        wasm_module: wasm_byte,
        arg: arg_encode,
    };

    match api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config, ),
    ).await {
        Ok(o) => o,
        Err((code, msg)) => {
            ic_cdk::api::trap(&msg);
        }
    };

    // match api::call::call(
    //     Principal::management_canister(),
    //     "install_code",
    //     (install_config,),
    // )
    //     .await
    // {
    //     Ok(x) => x,
    //     Err((code, msg)) => {
    //         ic_cdk::api::trap(&msg);
    //     }
    // };
}

pub async fn canister_status(id: Principal) -> CanisterStatus {
    let arg = StatusArgs { canister_id: id };

    let (status, ): (CanisterStatus, ) = match call(
        Principal::management_canister(),
        "canister_status",
        (arg, ),
    ).await {
        Ok(o) => o,
        Err((code, msg)) => {
            ic_cdk::api::trap(&msg);
        }
    };
    return status;
}

pub async fn update_setting(canister_id: Principal, controllers: Vec<Principal>) {
    let setting = CanisterSettings {
        controllers: Some(controllers),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };
    let update_setting = UpdateSettings {
        canister_id: canister_id,
        settings: setting,
    };

    let r: CallResult<((), )> = call(
        Principal::management_canister(),
        "update_settings",
        (update_setting, ),
    ).await;
    if let Err((code, msg)) = r {
        ic_cdk::api::trap(&msg);
    }
}