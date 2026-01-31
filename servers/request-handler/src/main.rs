use std::error::Error;
use std::ptr::addr_of;
use csv::ReaderBuilder;
use serde::Deserialize;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::data::bd_handler_server::{BdHandler, BdHandlerServer};
use crate::data::{HelloRequest, HelloResponse};
use crate::data::bd_handler_client::BdHandlerClient;

pub mod data {
    tonic::include_proto!("data");
}

#[derive(Debug, Deserialize, PartialEq)]
struct Prise {
    #[serde(rename = "<TICKER>")]
    ticker: String,
    #[serde(rename = "<PER>")]
    per: String,
    #[serde(rename = "<DATE>")]
    date: String,
    #[serde(rename = "<TIME>")]
    time: String,
    #[serde(rename = "<OPEN>")]
    open_prise: f64,
    #[serde(rename = "<HIGH>")]
    high_prize: f64,
    #[serde(rename = "<LOW>")]
    low_prize: f64,
    #[serde(rename = "<CLOSE>")]
    close_prize: f64,
    #[serde(rename = "<VOL>")]
    volume: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BdHandlerClient::connect("http://[::1]:50051")
        .await?;

    let request = tonic::Request::new(
        HelloRequest {
            name: "Rust-developer".into(),
        });

    let response = client.say_hello(request)
        .await?;

    println!("Ответ от сервера: {:?}", response.into_inner().message);

    Ok(())
}

fn load() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path("GLDRUB_TOM_260112_260113.csv")?;
    for record in rdr.deserialize() {
        let record: Prise = record?;
        println!("{:?}", record);
    }
    Ok(())
}