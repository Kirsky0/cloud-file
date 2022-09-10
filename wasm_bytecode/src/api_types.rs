use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize)]
pub struct WasmBytecode {
    pub module_name: String,
    pub bytecode: String,
    pub version: u32,
    pub remark: String,
    pub create_time: u64,
}

#[derive(CandidType, Deserialize,Clone)]
pub struct InitArgs {
    pub controller: String,
}

#[derive(CandidType, Deserialize)]
pub struct EnvParam {
    pub controller: String,
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
pub enum Status {
    running,
    stopping,
    stopped,
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettings {
    pub controllers: Vec<Principal>,
    pub compute_allocation: u128,
    pub memory_allocation: u128,
    pub freezing_threshold: u128,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<u128>,
    pub memory_allocation: Option<u128>,
    pub freezing_threshold: Option<u128>,
}


#[derive(CandidType, Deserialize)]
pub struct UpdateSettings {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}

#[derive(CandidType, Deserialize)]
pub struct StatusArgs {
    pub canister_id: Principal,
}
