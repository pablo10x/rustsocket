
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::time::Duration;

#[tokio::main]

async fn main() {
    let mut a: i16 = 24;
    increment(&mut a, 20);
    println!("{}", a);
    my_async_function(5).await;
    println!("finshed all")
}

fn increment(num: &mut i16, add: i16) -> i16 {
    *num += add;
    *num
}
async fn my_async_function(time: u64) {
    // Your asynchronous code goes here
    println!("Doing some async work...");
    tokio::time::sleep(tokio::time::Duration::from_secs(time)).await; // Simulating asynchronous work
    println!("Async work completed!");
    tokio::time::sleep(tokio::time::Duration::from_secs(time)).await;
}
