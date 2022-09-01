use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use ic_cdk::api;
use ic_cdk::api::call::CallResult;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use base64;
use candid::{CandidType, Encode, Nat};


#[derive(CandidType, Deserialize)]
struct CanisterInstall {
    pub mode: InstallMode,
    pub canister_id: Principal,
    pub wasm_module: Vec<u8>,
    pub arg: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

pub async fn install_code(canister_id: Principal, wasm_byte: Vec<u8>)
{
    let empty_arg: Vec<u8> = Default::default();
    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id: canister_id,
        wasm_module: wasm_byte,
        arg: empty_arg,
    };

    let r: CallResult<((), )> = api::call::call(
        Principal::management_canister(),
        "install_code",
        (install_config, ),
    ).await;
    if let Err((code, msg)) = r {
        ic_cdk::api::trap(&msg);
    }
}

