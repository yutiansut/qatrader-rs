use serde::{Deserialize, Serialize};
use serde_json::Value;
use log::{debug, error, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peek {
    pub aid: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Broker {
    pub aid: String,
    pub brokers: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqLogin {
    pub aid: String,
    pub bid: String,
    pub user_name: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqOrder {
    pub aid: String,
    pub user_id: String,
    pub order_id: String,
    pub exchange_id: String,
    pub instrument_id: String,
    pub direction: String,
    pub offset: String,
    pub volume: i64,
    pub price_type: String,
    pub limit_price: f64,
    pub volume_condition: String,
    pub time_condition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqCancel {
    pub aid: String,
    pub user_id: String,
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQueryBank {
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQuerySettlement {
    pub aid: String,
    pub trading_day: i64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqChangePassword {
    pub aid: String,
    pub old_password: String,
    pub new_password: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqTransfer {
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String,
    pub amount: f64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RtnData {
    pub aid: String,
    pub data: Vec<String>,
}


pub fn parse_message(msg: String) -> Option<String> {
    let resx: Value = match serde_json::from_str(&msg) {
        Ok(data) => data,
        Err(e) => {
            error!("{:?}", e);
            return None;
        }
    };
    let topic = match resx["topic"].as_str() {
        Some(data) => data,
        None => return None
    };
    let data = match topic {
        "sendorder" => {
            let order_id = uuid::Uuid::new_v4();
            debug!("this is sendorder {:?}", resx);
            let order = ReqOrder {
                aid: "insert_order".to_string(),
                user_id: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                order_id: order_id.to_string(),
                exchange_id: resx["exchange_id"].as_str().unwrap().parse().unwrap(),
                instrument_id: resx["code"].as_str().unwrap().parse().unwrap(),
                direction: resx["order_direction"].as_str().unwrap().parse().unwrap(),
                offset: resx["order_offset"].as_str().unwrap().parse().unwrap(),
                volume: resx["volume"].as_i64().unwrap(),
                price_type: "LIMIT".to_string(),
                limit_price: resx["price"].as_f64().unwrap(),
                volume_condition: "ANY".to_string(),
                time_condition: "GFD".to_string(),
            };
            let b = serde_json::to_string(&order).unwrap();
            debug!("Pretend to send {:?}", b);
            b
        }
        "cancel_order" => {
            let cancelorder = ReqCancel {
                aid: "cancel_order".to_string(),
                user_id: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                order_id: resx["order_id"].as_str().unwrap().parse().unwrap(),
            };
            let b = serde_json::to_string(&cancelorder).unwrap();
            debug!("Pretend to send cancel {:?}", b);

            b
        }
        "transfer" => {
            let transfermsg = ReqTransfer {
                aid: "req_transfer".to_string(),
                bank_id: resx["bank_id"].as_str().unwrap().parse().unwrap(),
                future_account: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                future_password: resx["future_password"].as_str().unwrap().parse().unwrap(),
                bank_password: resx["bank_password"].as_str().unwrap().parse().unwrap(),
                currency: "CNY".to_string(),
                amount: resx["account_cookie"].as_f64().unwrap(),
            };
            let b = serde_json::to_string(&transfermsg).unwrap();
            debug!("Pretend to send transfer {:?}", b);
            b
        }
        "query_settlement" => {
            let qsettlementmsg = ReqQuerySettlement {
                aid: "qry_settlement_info".to_string(),
                trading_day: resx["trading_day"].as_i64().unwrap(),
            };
            let b = serde_json::to_string(&qsettlementmsg).unwrap();
            debug!("Pretend to send QuerySettlement {:?}", b);
            b
        }
        "query_bank" => {
            let qbankmsg = ReqQueryBank {
                aid: "qry_bankcapital".to_string(),
                bank_id: resx["bank_id"].as_str().unwrap().parse().unwrap(),
                future_account: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                future_password: resx["future_password"].as_str().unwrap().parse().unwrap(),
                bank_password: resx["bank_password"].as_str().unwrap().parse().unwrap(),
                currency: "CNY".to_string(),
            };
            let b = serde_json::to_string(&qbankmsg).unwrap();
            debug!("Pretend to send QueryBank {:?}", b);
            b
        }
        "change_password" => {
            let changepwdmsg = ReqChangePassword {
                aid: "change_password".to_string(),
                old_password: resx["old_password"].as_str().unwrap().parse().unwrap(),
                new_password: resx["new_password"].as_str().unwrap().parse().unwrap(),
            };
            let b = serde_json::to_string(&changepwdmsg).unwrap();
            debug!("Pretend to send ChangePassword {:?}", b);
            b
        }
        "peek" => {
            let peek = Peek { aid: "peek_message".to_string() };
            let b = serde_json::to_string(&peek).unwrap();
            debug!("Pretend to send Peek {:?}", b);
            b
        }
        "login" => {
            let login = ReqLogin {
                aid: "req_login".to_string(),
                bid: resx["bid"].as_str().unwrap().parse().unwrap(),
                user_name: resx["user_name"].as_str().unwrap().parse().unwrap(),
                password: resx["password"].as_str().unwrap().parse().unwrap(),
            };
            let b = serde_json::to_string(&login).unwrap();
            debug!("Pretend to send Login {:?}", b);
            b
        }
        _ => {
            warn!("non receive! {:?}", resx);
            return None;
        }
    };
    Some(data)
}