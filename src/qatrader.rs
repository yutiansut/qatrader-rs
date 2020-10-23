use chrono::{Local, DateTime};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use qifi_rs::{QIFI, Account, Order, Position, Trade, Transfer, BankDetail};
use websocket::OwnedMessage;
use crossbeam_channel::Sender;

use crate::qaeventmq::MQPublish;
use crate::xmsg::XReqQueryBank;
use crate::qamongo::{struct_to_doc, get_collection, update_qifi};

pub struct QATrader {
    pub qifi: QIFI,
    pub last_update_time: DateTime<Local>,
    pub ws_sender: Sender<OwnedMessage>,
    pub pub_transaction: MQPublish,
}

impl QATrader {
    pub fn new(ws_sender: Sender<OwnedMessage>, account_cookie: String, password: String, wsuri: String, broker_name: String, portfolio: String,
               eventmq_ip: String, ping_gap: i32, bank_password: String, capital_password: String, taskid: String,
    ) -> Self {
        Self {
            last_update_time: Local::now(),
            pub_transaction: MQPublish::new(&eventmq_ip),
            ws_sender,
            qifi: QIFI {
                account_cookie,
                password,
                portfolio,
                broker_name,
                eventmq_ip,
                capital_password,
                bank_password,
                pub_host: "127.0.0.1".to_string(),
                taskid,
                trade_host: "127.0.0.1".to_string(),
                wsuri,
                status: 200,
                ping_gap,
                ..QIFI::default()
            },
        }
    }


    pub fn rtn_data_handler(&mut self, data: &Value) {
        let account_cookie = self.qifi.account_cookie.clone();
        self.last_update_time = Local::now();
        self.qifi.updatetime = self.last_update_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let new_message = data[&account_cookie].clone();
        if let Some(s) = new_message.get("session") {
            let trading_day = new_message["session"]["trading_day"].as_str().unwrap();
            self.qifi.trading_day = trading_day.to_string();
        }
        if let Some(a) = new_message.get("accounts") {
            if let Some(cny) = a.get("CNY") {
                let acc: Account = serde_json::from_value(cny.clone()).unwrap();
                self.qifi.accounts = acc;
                //

                if self.qifi.accounts.WithdrawQuota == 0.0 {
                    self.qifi.accounts.WithdrawQuota = self.qifi.accounts.available
                }
            }
        }
        if let Some(investor_name) = new_message.get("investor_name") {
            self.qifi.investor_name = investor_name.as_str().unwrap().to_string();
        }

        if let Some(positions) = new_message.get("positions") {
            let new_pos: HashMap<String, Position> = serde_json::from_value(positions.clone()).unwrap();
            for (key, val) in new_pos {
                self.qifi.positions.insert(key, val);
            }
        }
        if let Some(orders) = new_message.get("orders") {
            let new_ord: BTreeMap<String, Order> = serde_json::from_value(orders.clone()).unwrap();
            for (key, val) in new_ord {
                self.qifi.orders.insert(key, val);
            }
        }
        if let Some(transfers) = new_message.get("transfers") {
            let new_tranf: BTreeMap<String, Transfer> = serde_json::from_value(transfers.clone()).unwrap();
            for (key, val) in new_tranf {
                self.qifi.transfers.insert(key, val);
            }
        }
        if let Some(banks) = new_message.get("banks") {
            let new_bank: BTreeMap<String, BankDetail> = serde_json::from_value(banks.clone()).unwrap();
            for (key, val) in new_bank.clone() {
                self.qifi.banks.insert(key, val);
            }
            if !new_bank.is_empty() {
                for (key, val) in new_bank {
                    if key.eq("") {
                        let x = XReqQueryBank {
                            topic: "query_bank".to_string(),
                            aid: "qry_bankcapital".to_string(),
                            bank_id: val.id.clone(),
                            future_account: self.qifi.account_cookie.clone(),
                            future_password: self.qifi.capital_password.clone(),
                            bank_password: self.qifi.bank_password.clone(),
                            currency: "CNY".to_string(),
                        };
                        let b = serde_json::to_string(&x).unwrap();
                        self.ws_sender.send(OwnedMessage::Text(b));
                    }
                    self.qifi.bankid = val.id.clone();
                    self.qifi.bankname = val.name.clone();
                    self.qifi.money = val.fetch_amount.clone();
                    break;
                }
            }
        }
        if let Some(trades) = new_message.get("trades") {
            let new_trad: BTreeMap<String, Trade> = serde_json::from_value(trades.clone()).unwrap();

            for (key, val) in new_trad.iter() {
                if self.qifi.trades.get(key).is_none() {
                    self.pub_transaction.publish_routing("QATRANSACTION", serde_json::to_string(val).unwrap(), &self.qifi.account_cookie)
                }
            }

            for (key, val) in new_trad {
                self.qifi.trades.insert(key, val);
            }
        }

        update_qifi(self.qifi.clone());
    }


    pub fn qry_settlement_info_handler(&mut self, data: &Value) {
        let reportdate = data["trading_day"].as_str().unwrap();
        if let Some(x) = self.qifi.settlement.get_mut(reportdate) {
            *x = data["settlement_info"].as_str().unwrap().to_string()
        }
        update_qifi(self.qifi.clone());
    }

    pub fn notify_handler(&mut self, data: &Value) {
        //  """{'N8': {'type': 'MESSAGE', 'level': 'INFO', 'code': 0, 'content': '登录成功'}"""
        let ni = data.as_object().unwrap();
        for (k, v) in ni {
            let mess = v["content"].as_str().unwrap().to_string();
            self.qifi.event.insert(Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), mess.clone());
            if mess.contains("修改密码成功") {} else if mess.contains("转账成功") {} else if mess.contains("这一时间段不能转账") {} else if mess.contains("银行账户余额不足") {} else if mess.contains("下单成功") {} else if mess.contains("撤单成功") {} else if mess.contains("用户登录失败") {
                self.qifi.status = 600;
            }
        }

        update_qifi(self.qifi.clone());
    }

    pub fn sync(&mut self, msg: String) {
        if let Ok(val) = serde_json::from_str::<Value>(&msg) {
            if let Some(aid) = val["aid"].as_str() {
                let m = &val["data"][0]["trade"];
                let n = &val["data"][0]["notify"];
                match aid {
                    "rtn_data" if !m.is_null() => {
                        self.rtn_data_handler(m)
                    }
                    "rtn_data" if !n.is_null() => {
                        self.notify_handler(n)
                    }
                    "qry_settlement_info" if !m.is_null() => {
                        self.qry_settlement_info_handler(m)
                    }
                    _ => ()
                }
            }
        }
    }
}