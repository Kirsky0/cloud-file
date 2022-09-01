use ic_cdk::export::Principal;

pub struct User {
    user_main_canister: Principal,
    username: String,
}

impl User {
    pub fn new(user_main_canister: Principal, username: String) -> User {
        let u = User {
            user_main_canister,
            username,
        };
        return u;
    }
    pub fn get_username(&self) -> &String
    {
        return &self.username;
    }
    pub fn get_user_main_canister(&self) -> &Principal
    {
        return &self.user_main_canister;
    }
}