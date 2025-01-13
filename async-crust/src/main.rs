#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead, process::Output};

// #[tokio::main]
// async fn main() {
fn main() {
    let runtime = tokio::runtime::Runtime::new();
    // It's the infinite loop of waiting for the next operation to finish but smarter
    // Registers everything in internal state and when code yields back here
    // It registers all operations in the OS and goes to sleep until system tells
    // that some operation was finished. And after that it yields back to the place where it was
    // finishedd
    runtime.block_on(async {
        // you can do async code here
        let x = foo1().await;
        let y = foo2().await;
    });

    println!("Hello, world!");
    let x1 = foo1();
    let x2 = foo2();

    // This one how we can do it without async and it's kinda okay but messy
    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(&std::io::stdin());
        for line in x.lines() {
            // do something on user input
        }
    });
    let read_from_network = std::thread::spawn(move || {
        let mut x = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
        while let Ok(stream) = x.accept() {
            // do something on stream
            let handler = std::thread::spawn(move || {
                handle_connection(stream);
            })
        }
    });

    // With the futures
    let mut network = read_from_network();
    let mut terminal = read_from_terminal();

    let mut f1 = tokio::fs::File::open("file1");
    let mut f2 = tokio::fs::File::open("file2");
    let copy = tokio::io::copy(&mut f1, &mut f2);

    // Some global loop with Future that handles all async code below and try to run it
    // But in practice it does not do like that it's do stuff like Epoll loop on Linux
    loop {
        select! {
            stream <- (&mut network).await => {
                // do something on stream
            }
            line <- (&mut terminal).await => {
                // do something with line
            }
            foo <- foo2().await => {}
            _ <- copy.await => {}
        }

        // For example stream was finished and we don't have loop here
        // Add only some bytes were copied from f1 to f2
        // And this things could happen and you need to be careful with it
        //
        // So need to be careful with select and think what will happen if one branch will be
        // executed but not finished and another one will eventually be finished
    }
}

fn cancellation(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = Result<usize, String>> {
    async {
        race! {
            done <- read_to_string("file").await => {
                // done
            }
            cancel <- cancel.await => {
                // cancel and return error
                return Err("canceled before read is finished")
            }
        }
        Ok(0)
    }
}

// foo1 === foo2
async fn foo1() -> usize {
    println!("foo1");
    0
}

fn foo2() -> impl Future<Output = usize> {
    // We can do it here, but not in foo1
    println!("pre foo2");

    async {
        println!("foo1 exec");
        // will not do anything
        foo1();
        println!("foo2");
        0
    }
}

fn mental_model() -> impl Future<Output = usize> {
    async {
        // let x = read_to_string("file").await;
        
        let fut = read_to_string("file");
        let x = loop {
            if let Some(result) = fut.try_check_completed() {
                break result;
            } else {
                fut.try_make_progress();
                // yield to the up level await and tries to make progress again;
                yield;
            }
        }


        0
    }
}

