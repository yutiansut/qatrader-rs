extern crate mongodb;

pub use mongodb::{
    bson::{Bson, doc, Document},
    error::Result,
    options::{
        DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateModifications,
        UpdateOptions,
    },
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection, Cursor, Database},
};

pub fn query_account(mongo_ip:String, account_cookie:String) {
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
    let mut cursor = coll.find(doc.clone(), None)
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


struct Qifiaccount{
    account_cookie: String
    
}

pub fn update_account(mongo_ip:String, account_cookie:String,) {
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
    let mut cursor = coll.find(doc.clone(), None)
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
