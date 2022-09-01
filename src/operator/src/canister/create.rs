use candid::CandidType;
use ic_cdk::api;
use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Deserialize, Clone)]
struct CreateResult {
    pub canister_id: Principal,
}

#[derive(CandidType, Clone, Deserialize)]
struct CanisterSettings {
    controllers: Vec<Principal>,
}

pub async fn create_canister() -> Principal {
    let setting = CanisterSettings {
        controllers: vec![Principal::from_text("auflz-yecw4-vr476-5dsua-dogvv-ln22t-ziyab-egeep-osas4-cpesk-nae").unwrap(), Principal::management_canister()],
    };
    let cycles: u128 = 800_000_000_000;
    let r: CallResult<(CreateResult, )> = api::call::call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (setting, ),
        cycles,
    ).await;
    if let Err((code, msg)) = r {
        ic_cdk::api::trap(&msg);
    }
    return r.unwrap().0.canister_id;

    // let (create_result, ): (CreateResult, ) = match api::call::call_with_payment128(
    //     Principal::management_canister(),
    //     "create_canister",
    //     (),
    //     cycles,
    // )
    //     .await
    // {
    //     Ok(x) => x,
    //     Err((code, msg)) => {
    //         return Err(format!(
    //             "An error happened during the _create_canister_call: {}: {}",
    //             code as u8, msg
    //         ));
    //     }
    // };
    //
    // Ok(create_result)
}