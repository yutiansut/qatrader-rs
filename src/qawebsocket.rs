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
    aid: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Broker {
    aid: String,
    brokers: Vec<String>,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqLogin {
    aid: String,
    bid: String,
    user_name: String,
    password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqOrder {
    aid: String,
    user_id:String,
    order_id: String,
    exchange_id: String,
    instrument_id: String,
    direction: String,
    offset: String,
    volume: String,
    price_type: String,
    limit_price: String,
    volume_condition: String,
    time_condition: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqCancel {
    aid: String,
    user_id:String,
    order_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQueryBank {
    aid: String,
    bank_id: String,
    future_account: String,
    future_password: String,
    bank_password: String,
    currency: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqQuerySettlement {
    aid: String,
    trading_day: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqChangePassword {
    aid: String,
    old_password: String,
    new_password: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqTransfer {
    aid: String,
    bank_id: String,
    future_account: String,
    future_password: String,
    bank_password: String,
    currency: String,
    amount: f64

}


#[derive(Serialize, Deserialize, Debug)]
pub struct RtnData {
    aid: String,
    data: Vec<String>
}
