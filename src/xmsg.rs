use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct XPeek {
    pub topic: String,
    pub aid: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct XReqLogin {
    pub topic: String,
    pub aid: String,
    pub bid: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XReqQueryBank {
    pub topic: String,
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String,
}