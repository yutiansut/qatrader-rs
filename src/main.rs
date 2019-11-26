extern crate mongodb;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use lapin::{
    message::DeliveryResult, options::*, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties, ConsumerDelegate,
};
use log::info;

#[derive(Clone, Debug)]
struct Subscriber {
    channel: Channel,
}

impl ConsumerDelegate for Subscriber {
    fn on_new_delivery(&self, delivery: DeliveryResult) {
        if let Ok(Some(delivery)) = delivery {
            self.channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .wait()
                .expect("basic_ack");
        }
    }
}

fn main() {
    query_account("192.168.2.24".to_string(), "288870".to_string());
    connect_mq("192.168.2.24".to_string())
}

fn query_account(mongo_ip:String, account_cookie:String) {
    let client = Client::connect(&mongo_ip, 27017)
    .expect("Failed to initialize standalone client.");

    let coll = client.db("QAREALTIME").collection("account");

    let doc = doc! {
        "account_cookie": account_cookie
    };

    // Insert document into 'test.movies' collection
    // coll.insert_one(doc.clone(), None)
    //     .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("accounts") {
            Some(&Bson::Document(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}

fn connect_mq(eventmq_amqp:String) {
    info!("{}", eventmq_amqp);
 
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:admin@127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .wait()
        .expect("connection error");

    info!("CONNECTED");

    let channel_a = conn.create_channel().wait().expect("create_channel");
    let channel_b = conn.create_channel().wait().expect("create_channel");

    channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_declare");
    let queue = channel_b
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_declare");

    info!("will consume");
    channel_b
        .clone()
        .basic_consume(
            &queue,
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("basic_consume")
        .set_delegate(Box::new(Subscriber { channel: channel_b }));

    let payload = b"Hello world!";

    loop {
        channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .wait()
            .expect("basic_publish");
    }
}