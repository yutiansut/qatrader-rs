//! Simple websocket client.
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::{thread};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    Client,
};
use futures::{
    lazy,
    stream::{SplitSink, Stream},
    Future,
};


pub fn wsmain(wsuri:String, user_name:String, password:String) {

    let sys = actix::System::new("ws-example");


    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws(wsuri)
            .connect()
            .map_err(|e| {
                println!("QAConnection Error: {}", e);
            })
            .map(|(response, framed)| {
                println!("{:?}", response);
                let (sink, stream) = framed.split();
                let addr = ChatClient::create(|ctx| {
                    ChatClient::add_stream(stream, ctx);
                    ChatClient(SinkWrite::new(sink, ctx))
                });

                // start console loop loop
                thread::spawn(move || {
                    let login = ReqLogin {
                        aid: "req_login".to_string(),
                        bid: "QUANTAXIS".to_string(),
                        user_name,
                        password,};

                    let b = serde_json::to_string(&login).unwrap();
                    addr.do_send(ClientCommand(b));
                });
            })
    }));

    let _ = sys.run();
}

use crate::qaeventmq;
use crate::qaeventmq::{Publisher,Subscriber, Callback};

pub fn QAtradeR(user_name : String, password: String, broker: String,
                mongo_uri:String, ws_uri: String, eventmq_uri: String,) {


    let sys = actix::System::new("qatrader");


    /// 需要一个缓冲的Queue, 来进行线程中间消息的分发
    /// 
    /// rabbitmq.QAORDER_ROUTER.account_cookie
    /// recv => meessage ==> queue
    /// queue ==> to websocket ==> send
    /// websocket ==> recv => save to database ==> send new requests
    /// 
    /// 
    /// database is the only interface of account message exploruse

    impl Callback for qaeventmq::QAEventMQ {
        fn callback(&mut self, message: String) -> Option<i32> {
            ///do somehing
            /// 
            let resx:Value = serde_json::from_str(&message).unwrap();

            if resx["topic"] == "send_order" {
                println!("1")
            }else if resx["topic"] == "cancel_order" {
                println!("Cancel Order")
            }          
            Some(1)

        }
    }
    let mut client = qaeventmq::QAEventMQ{
        amqp: eventmq_uri,
        exchange: "QAORDER_ROUTER".to_string(),
        model: "direct".to_string(),
        routing_key: user_name.clone(),
    };

    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws(ws_uri)
            .connect()
            .map_err(|e| {
                println!("QAConnection Error: {}", e);
            })
            .map(|(response, framed)| {
                println!("{:?}", response);
                let (sink, stream) = framed.split();
                let addr = ChatClient::create(|ctx| {
                    ChatClient::add_stream(stream, ctx);
                    ChatClient(SinkWrite::new(sink, ctx))
                });


                // start console loop loop
                thread::spawn(move || {
                    let login = ReqLogin {
                        aid: "req_login".to_string(),
                        bid: broker.clone(),
                        user_name: user_name.clone(),
                        password: password.clone(),};

                    let b = serde_json::to_string(&login).unwrap();
                    addr.do_send(ClientCommand(b));

                    thread::spawn(move || {
                        client.subscribe_routing();
                    });





                });
            })
    }));

    let _ = sys.run();


}




pub struct ChatClient<T>(SinkWrite<SplitSink<Framed<T, Codec>>>)
where
    T: AsyncRead + AsyncWrite;

#[derive(Message)]
pub struct ClientCommand(String);

impl<T: 'static> Actor for ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl<T: 'static> ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.write(Message::Ping(String::new())).unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

impl<T: 'static> StreamHandler<Frame, WsProtocolError> for ChatClient<T>
    where
        T: AsyncRead + AsyncWrite,
{
    fn handle(&mut self, msg: Frame, _ctx: &mut Context<Self>) {

        if let Frame::Text(txt) = msg {
            let res =  txt.unwrap();
            let xu = std::str::from_utf8(&res).unwrap();
            println!("{:?}",xu);
            let resx:Value = serde_json::from_str(&xu).unwrap();

            let aid = resx["aid"].to_string();
            let aid_patten = aid.as_str();
            let peek = Peek { aid: "peek_message".to_string()};

            println!("{:?}",aid_patten);
            match aid_patten {
                "\"rtn_brokers\"" => {

                    println!("{:?}", peek);
                    let b = serde_json::to_string(&peek).unwrap();
                    println!("{:?}",b);
                    self.0.write(Message::Text(b)).unwrap();

//
//                    println!("{:?}", login);
//                    let b = serde_json::to_string(&login).unwrap();
//                    println!("{:?}",b);
//                    self.0.write(Message::Text(b)).unwrap();
                },
                "\"rtn_data\"" | "\"rtn_condition_orders\"" => {
                    println!("xxx");
                    let b = serde_json::to_string(&peek).unwrap();
                    println!("{:?}",b);
                    self.0.write(Message::Text(b)).unwrap();
                },
                _ => println!("blahh blahhh"),
            }
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
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

use serde_json::value::Value;


/// Handle stdin commands
impl<T: 'static> Handler<ClientCommand> for ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Result = ();
    
    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0)).unwrap();
    }
}

/// Handle server websocket messages


impl<T: 'static> actix::io::WriteHandler<WsProtocolError> for ChatClient<T> where
    T: AsyncRead + AsyncWrite
{
}