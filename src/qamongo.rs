use mongodb::{Client, Collection};
use mongodb::options::UpdateOptions;
use serde::Serialize;
use bson::{to_bson, Document, doc};
use lazy_static::lazy_static;
use crate::config::CONFIG;
use qifi_rs::QIFI;


lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

pub fn struct_to_doc<T>(value: T) -> Document
    where
        T: Serialize + std::fmt::Debug,
{
    to_bson(&value)
        .unwrap()
        .as_document()
        .unwrap()
        .to_owned()
}

fn create_mongo_client() -> Client {
    Client::with_uri_str(&CONFIG.common.database_ip)
        .expect("Failed to initialize client. Please check the uri first")
}

pub fn get_collection(coll_name: &str) -> Collection {
    MONGO.database("QAREALTIME").collection(coll_name)
}


pub fn update_qifi(qifi: QIFI) {
    let account_cookie = qifi.account_cookie.clone();
    let slice = struct_to_doc(qifi);
    let options = UpdateOptions::builder().upsert(true).build();
    get_collection("account").update_one(doc! {
            "account_cookie": account_cookie}, doc! { "$set": slice}, options).unwrap();
}