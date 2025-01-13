#![allow(dead_code, unused_variables)]

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // tokio::sync::Mutex
    let x = Arc::new(Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let mut x = x1.lock().unwrap();
            tokio::fs::read_to_string("file").await;
            // potential deadlock if we use regular Mutex
            *x += 1;
        }
    });
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            // but if we use tokio mutex this one will be aware 
            // and will be just blocks, but yield instead
            *x2.lock().unwrap() += 1;
        }
    });
}

async fn foo() {}
