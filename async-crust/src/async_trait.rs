#![allow(dead_code, unused_variables)]

use std::time::Duration;

use async_trait::async_trait;

fn main() {}

struct Response;
struct Request;

#[async_trait]
trait Service {
    async fn call(&mut self, _: Request) -> Response;
                                        // Pin<Box<dyn Future<Output = Response>>>
 
}

struct X;
#[async_trait]
impl Service for X {
    async fn call(&mut self, _: Request) -> Response {
        Response
        // Box::pin(async move { Response })
    
    }
}

struct Y;
#[async_trait]
impl Service for Y {
    async fn call(&mut self, _: Request) -> Response {
        let z = [0; 1024];
        tokio::time::sleep(Duration::from_secs(100)).await;
        drop(z);
        Response
    }
}

fn foo(x: &mut dyn Service) {
    // The size of fut depends on what local stack variables are used in the
    // implementation of the method
    // That's why we need to use third-party macro async_trait
    let fut = x.call(Request);
}
