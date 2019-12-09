//! Simple websocket client.
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::thread;


use wsq::{connect, Handler, Handshake, Message, Result};
use wsq::Sender;

pub struct QAtradeR{
    pub out: Sender,
    pub user_name : String,
    pub password: String,
    pub broker: String,

}



impl Handler for QAtradeR{


    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("Hello to peer: {}", handshake.peer_addr.unwrap());

        let login = ReqLogin {
            aid: "req_login".to_string(),
            bid: self.broker.clone(),
            user_name: self.user_name.clone(),
            password: self.password.clone(),};

        let b = serde_json::to_string(&login).unwrap();
        println!("{}", b);
        self.out.send(b).unwrap();

        Ok(())
    }




    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Message::Text(message_text) = msg {
            println!("{}", message_text);
            let peek = Peek { aid: "peek_message".to_string()};
            let b = serde_json::to_string(&peek).unwrap();
            self.out.send(b).unwrap();

        }


        Ok(())
    }




}



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
    pub password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqOrder {
    pub aid: String,
    pub user_id:String,
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
    pub user_id:String,
    pub order_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQueryBank {
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQuerySettlement {
    pub aid: String,
    pub trading_day: i64
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqChangePassword {
    pub aid: String,
    pub old_password: String,
    pub new_password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqTransfer {
    pub aid: String,
    pub bank_id: String,
    pub future_account: String,
    pub future_password: String,
    pub bank_password: String,
    pub currency: String,
    pub amount: f64

}


#[derive(Serialize, Deserialize, Debug)]
pub struct RtnData {
    pub aid: String,
    pub data: Vec<String>
}
