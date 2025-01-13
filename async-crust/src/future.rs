#![allow(dead_code, unused_variables)]

use std::future::Future;

#[tokio::main]
async fn main() {
    let mut x: StateMachine = foo();
    // sortof this, not particullary desuragring, but could have this mental model in place
    // StateMachine::await(&mut x);
    
    // you are memcopy x with 102000 * 8 bytes and entire state machine
    // with all nested futures state machines
    bar(x);


    // solution is to do heap allocation instead
    let mut x = Box::pin(foo());
    bar(x);
    
    // Also it's a good reason to use spawn
    // It returns pointer to the future but not the future itself
}

async fn bar(_: impl Future) {}

enum StateMachine {
    Chunk1 {
        x: [u8; 102400],
        fut: tokio::fs::ReadIntoFuture<'x>,
    },
    Chunk2 {},
}

// fn foo() -> impl Future<Output = ()> /* StateMachine */ 
async fn foo() {
    // chunk 1
    {
        let mut x = [0; 102400];
        let z = vec![];
        let fut = tokio::fs::read_into("file.dat", &mut x[..]).await;
    }

    // fut.await
    yield; // really: return
           // so where the x stored??? It stack? But if it's on stack than it should be dropped here
           // so futures has sorta enum that was generated with kinda state machine that stores all the variables

    // chunk 2
    {
        let n = self.fut.output();
        println!("{:?}", self.x[..n]);
        // this state should be stored in the state machine
        some_library::execute().await;
    }
}
