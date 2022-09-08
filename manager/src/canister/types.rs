use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub struct StoreUnit {
    pub canister_id: Principal,
    pub controllers: Vec<Principal>,
    pub size: u128,
    pub cycle: u128,
    pub installed_code: bool,
}


#[derive(CandidType, Deserialize, Clone)]
pub struct CreateResult {
    pub canister_id: Principal,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterInstall {
    pub mode: InstallMode,
    pub canister_id: Principal,
    pub wasm_module: Vec<u8>,
    pub arg: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(CandidType, Deserialize)]
pub struct InstallArgs {
    pub controller: String,
}
