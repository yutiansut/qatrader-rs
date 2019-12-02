//! Simple websocket client.
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::{io, thread};

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


pub fn wsmain(wsuri:String) {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
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

                // start console loop
                thread::spawn(move || loop {
                    let mut cmd = String::new();
                    if io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }
                    addr.do_send(ClientCommand(cmd));
                });
            })
    }));

    let _ = sys.run();
}

struct ChatClient<T>(SinkWrite<SplitSink<Framed<T, Codec>>>)
where
    T: AsyncRead + AsyncWrite;

#[derive(Message)]
struct ClientCommand(String);

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

#[derive(Serialize, Deserialize, Debug)]
struct Peek {
    aid: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Broker {
    aid: String,
    brokers: Vec<String>,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqLogin {
    aid: String,
    bid: String,
    user_name: String,
    password: String
}


#[derive(Serialize, Deserialize, Debug)]
struct ReqOrder {
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
struct ReqCancel {
    aid: String,
    user_id:String,
    order_id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqQueryBank {
    aid: String,
    bank_id: String,
    future_account: String,
    future_password: String,
    bank_password: String,
    currency: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqQuerySettlement {
    aid: String,
    trading_day: i32
}


#[derive(Serialize, Deserialize, Debug)]
struct ReqChangePassword {
    aid: String,
    old_password: String,
    new_password: String
}


#[derive(Serialize, Deserialize, Debug)]
struct ReqTransfer {
    aid: String,
    bank_id: String,
    future_account: String,
    future_password: String,
    bank_password: String,
    currency: String,
    amount: f64

}


#[derive(Serialize, Deserialize, Debug)]
struct RtnData {
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
            let login = ReqLogin { 
                aid: "req_login".to_string(),
                bid: "simnow".to_string(),
                user_name: "133495".to_string(),
                password: "QCHL1234".to_string()};
            println!("{:?}",aid_patten);
            // println!("{:?}",recv_broker);
            // assert_eq!(aid, Some("rtn_brokers"));
            // assert_eq!(aid, Some("rtn_data"));
            match aid_patten {
                "\"rtn_brokers\"" => {
                    
                    println!("{:?}", peek);
                    let b = serde_json::to_string(&peek).unwrap();
                    println!("{:?}",b);
                    self.0.write(Message::Text(b)).unwrap();
        
 
                    println!("{:?}", login);
                    let b = serde_json::to_string(&login).unwrap();
                    println!("{:?}",b);
                    self.0.write(Message::Text(b)).unwrap();
                },
                "\"rtn_data\"" | "\"rtn_condition_orders\"" => {
                    println!("xxx");
                    let b = serde_json::to_string(&peek).unwrap();
                    println!("{:?}",b);
                    self.0.write(Message::Text(b)).unwrap();
                },
                _ => println!("blahh blahhh"),
            }
            // let u: Broker = serde_json::from_str(&xu).unwrap();
            // println!("{}", serde_json::to_string(&u).unwrap());

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

impl<T: 'static> actix::io::WriteHandler<WsProtocolError> for ChatClient<T> where
    T: AsyncRead + AsyncWrite
{
}