use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize)]
pub struct StoreUnit {
    pub canister_id: Principal,
    pub controllers: Vec<Principal>,
    pub size: u128,
    pub cycle: u128,
    pub installed_code: bool,
}


#[derive(CandidType, Deserialize)]
pub struct CreateResult {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<u128>,
    pub memory_allocation: Option<u128>,
    pub freezing_threshold: Option<u128>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateCanisterArgs {
    pub settings: Option<CanisterSettings>,
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
    install,
    reinstall,
    upgrade,
}

#[derive(CandidType, Deserialize)]
pub struct InstallArgs {
    pub controller: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateArgs {
    pub controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct StatusArgs {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatus {
    pub status: Status,
    pub settings: DefiniteCanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub memory_size: u128,
    pub cycles: u128,
    pub idle_cycles_burned_per_day: u128,
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettings {
    pub controllers: Vec<Principal>,
    pub compute_allocation: u128,
    pub memory_allocation: u128,
    pub freezing_threshold: u128,
}

#[derive(CandidType, Deserialize)]
pub enum Status {
    running,
    stopping,
    stopped,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateSettings {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}