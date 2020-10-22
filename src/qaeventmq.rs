use serde_json::Value;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable, QueueDeclareOptions, Result, Publish, Channel};
use log::{info, warn, error};
use crossbeam_channel::{Sender, Receiver};
use websocket::OwnedMessage;


pub struct QAEventMQ {
    pub amqp: String,
    pub exchange: String,
    pub routing_key: String,
}

impl QAEventMQ {
    pub fn new(
        amqp: String,
        exchange: String,
        routing_key: String,
    ) -> Self {
        Self {
            amqp,
            exchange,
            routing_key,
        }
    }

    pub fn consume_direct(&self, sender: Sender<OwnedMessage>) -> Result<()> {
        let mut connection = Connection::insecure_open(&self.amqp)?;
        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(
            ExchangeType::Direct,
            &self.exchange,
            ExchangeDeclareOptions {
                durable: false,
                auto_delete: false,
                internal: false,
                arguments: Default::default(),
            },
        )?;
        let queue = channel.queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
        )?;
        info!("[{}] Receiving...", self.routing_key);

        queue.bind(&exchange, self.routing_key.clone(), FieldTable::new())?;

        let consumer = queue.consume(ConsumerOptions {
            no_ack: true,
            ..ConsumerOptions::default()
        })?;

        for (_i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let msg = delivery.body.clone();
                    let foo = String::from_utf8(msg).unwrap();
                    sender.send(OwnedMessage::Text(foo));
                }
                other => {
                    warn!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        connection.close()
    }
}


pub struct MQPublish {
    pub conn: Connection,
    pub channel: Channel,
}

impl MQPublish {
    pub fn new(amqp: &str) -> Self {
        let mut connection = Connection::insecure_open(amqp).unwrap();
        let channel = connection.open_channel(None).unwrap();
        Self {
            conn: connection,
            channel,
        }
    }
    pub fn publish_topic(&mut self, exchange_name: &str, context: String, routing_key: &str) {
        let exchange = self
            .channel
            .exchange_declare(
                ExchangeType::Topic,
                exchange_name,
                ExchangeDeclareOptions::default(),
            )
            .unwrap();
        exchange
            .publish(Publish::new(context.as_bytes(), routing_key))
            .unwrap();
        //connection.close();
    }

    pub fn publish_routing(&mut self, exchange_name: &str, context: String, routing_key: &str) {
        let exchange = self
            .channel
            .exchange_declare(
                ExchangeType::Direct,
                exchange_name,
                ExchangeDeclareOptions::default(),
            )
            .unwrap();
        exchange
            .publish(Publish::new(context.as_bytes(), routing_key))
            .unwrap();
        //connection.close();
    }
}
