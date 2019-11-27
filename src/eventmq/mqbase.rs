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

#[warn(unused_imports)]
pub fn connect_mq(eventmq_amqp:String, _exchange:String, queue_name:String, context:String) {
    info!("{}", eventmq_amqp);

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://admin:admin@192.168.2.118:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .wait()
        .expect("connection error");

    info!("CONNECTED");

    let channel_a = conn.create_channel().wait().expect("create_channel");
    let channel_b = conn.create_channel().wait().expect("create_channel");

    channel_a
        .queue_declare(
            &queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_declare");
    let queue = channel_b
        .queue_declare(
            &queue_name,
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

    let payload = context.as_bytes();

    loop {
        channel_a
            .basic_publish(
                "",
                &queue_name,
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .wait()
            .expect("basic_publish");
    }
}