use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub struct WasmBytecode {
    pub module_name: String,
    pub bytecode: String,
    pub version: u32,
    pub remark: String,
    pub create_time: u64,
}

#[derive(CandidType, Deserialize,Debug,Clone)]
pub struct InstallArgs {
    pub controller: String,
}

#[derive(CandidType, Deserialize,Debug,Clone)]
pub struct EnvParam {
    pub controller: String,
}

