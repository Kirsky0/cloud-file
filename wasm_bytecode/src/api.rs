use std::ops::Deref;
use std::sync::{Mutex};

use candid::candid_method;
use candid::CandidType;
use ic_cdk::api::call::CallResult;
use ic_cdk::{call, id};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use rusqlite::{Connection};

use crate::api_types::{CanisterSettings, CanisterStatus, EnvParam, InitArgs, UpdateSettings, WasmBytecode, StatusArgs};

static CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE IF
	NOT EXISTS wasm_bytecode (
	module_name text PRIMARY KEY,
	bytecode blob NOT NULL,
	version integer NOT NULL,
	remark text NOT NULL,
	create_time integer NOT NULL)",
        (), // empty list of parameters.
    );
    Mutex::new(conn)
});

static ENV_PARAM: Lazy<Mutex<EnvParam>> = Lazy::new(|| {
    let empty = EnvParam { controller: "".to_string() };
    Mutex::new(empty)
});


#[init]
pub fn init() {
    let call_arg = ic_cdk::api::call::arg_data::<(Option<InitArgs>, )>().0;
    let mut env_param = ENV_PARAM.lock().unwrap();
    if call_arg.is_some()
    {
        env_param.controller = call_arg.unwrap().clone().controller;
        ic_cdk::print(format!("{:?}", &env_param.controller));
    }
}

#[update(name = "add_wasm_bytecode")]
#[candid_method(update)]
pub fn add_wasm_bytecode(module_name: String, bytecode: String, remark: String, create_time: u64)
{
    // let record: WasmBytecode = serde_json::from_str(&json).unwrap();

    let mut conn = CONN.lock().unwrap();
    let sql: &str = "INSERT INTO wasm_bytecode (module_name,bytecode,version,remark,create_time) VALUES (?1, ?2, ?3, ?4, ?5)";

    let mut version = exist_record_count(&module_name);
    version = version + 1;
    conn.execute(sql, rusqlite::params![module_name,bytecode,version,remark,create_time]);
}

fn exist_record_count(module_name: &String) -> usize
{
    let mut conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT  COUNT(module_name)  FROM wasm_bytecode WHERE module_name=? ").unwrap();
    let mut result_rows = stmt.query(rusqlite::params![module_name]).unwrap();
    let mut count: usize = 0;
    while let Some(row) = result_rows.next().unwrap() {
        count = row.get_unwrap(0);
    }
    return count;
}

#[query(name = "list")]
#[candid_method(query)]
pub fn list() -> Vec<WasmBytecode>
{
    let conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT module_name,version,remark,create_time FROM wasm_bytecode  ").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut records: Vec<WasmBytecode> = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let p = WasmBytecode {
            module_name: row.get(0).unwrap(),
            bytecode: "".to_string(),
            version: row.get(1).unwrap(),
            remark: row.get(2).unwrap(),
            create_time: row.get(3).unwrap(),
        };
        records.push(p)
    }
    return records;
}

// #[query(name = "get_controller")]
// #[candid_method(query)]
// pub fn get_controller() -> String
// {
//     return ENV_PARAM.lock().unwrap().deref().clone().controller;
// }


// #[update(name = "status")]
// #[candid_method(update)]
// pub async fn status() -> String {
//     let arg = StatusArgs { canister_id: id() };
//     let r: CallResult<(CanisterStatus, )> = call(
//         Principal::management_canister(),
//         "canister_status",
//         (arg, ),
//     ).await;
//     if let Err((code, msg)) = r {
//         ic_cdk::api::trap(&msg);
//     }
//     let status = r.unwrap().0;
//     return serde_json::to_string(&status).unwrap();
// }
//
// #[update(name = "update_setting")]
// #[candid_method(update)]
// pub async fn update_setting(mut controllers: Vec<Principal>) {
//     controllers.push(id());
//     let setting = CanisterSettings {
//         controllers: Some(controllers),
//         compute_allocation: None,
//         memory_allocation: None,
//         freezing_threshold: None,
//     };
//     let update_setting = UpdateSettings {
//         canister_id: id(),
//         settings: setting,
//     };
//
//     let r: CallResult<((), )> = call(
//         Principal::management_canister(),
//         "update_settings",
//         (update_setting, ),
//     ).await;
//     if let Err((code, msg)) = r {
//         ic_cdk::api::trap(&msg);
//     }
// }