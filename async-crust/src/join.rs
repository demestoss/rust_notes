#![allow(dead_code, unused_variables)]

use std::sync::{Arc, Mutex};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let files = (0..3)
        .map(|i| tokio::fs::read_to_string(format!("file{i}")))
        .collect::<Vec<_>>();

    // join from the futures crate, could be usefull for the small amount of things
    let (f1, f2, f3) = join!(files[0], files[1], files[2]);

    // if you care about the order of the futures
    // but it takes extra
    let file_bytes = try_join_all(files);
    file_bytes[0] == files[0];

    // If you don't care FuturesUnordered will be the solution
    // this is more efficient than try_join_all

    let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");

    // This one sucks because we only have one thread which will be always bisy with handling connections
    // if we have a lot of them and never will get connections from the array itself
    // to work with it
    // let mut connections = futures::stream::FuturesUnordered::new();
    // loop {
    //     select! {
    //         stream = &mut accept => {
    //             connections.push(handle_connection(stream.unwrap()));
    //         },
    //         _ = connections.await => {}
    //     }
    // }

    while let Ok(stream) = accept.await {
        // as is if we give it to the runtime
        // so now we have 2 "top-level" futures - this one and the top main one
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpListener) {
    let x = Arc::new(Mutex::new(vec![""]));
    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        x1.lock();
        0
    });
    assert_eq!(join_handle.await.unwrap(), 0);
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock();
    });
}
