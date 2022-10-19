extern crate core;

use dkregistry::v2::Client;
use std::{
    fmt::{Error, Result},
    result,
};
use tracing::{span, Level};
use tracing_subscriber;
use std::fs::File;
use futures_util::StreamExt;


#[async_std::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();
    foo().await;
}

async fn foo() -> bool {
    let host = "public.ecr.aws/g9h9x2a6/`";
    match Client::configure()
        .insecure_registry(false)
        .registry(host)
        .username(Some(String::from("springcomet")))
        .read_credentials(File::open("c:\\Users\\aviv\\tmp\\config.json").unwrap())
        .build()
    {
        Err(_) => {
            println!("err:build");
            true
        }
        Ok(c) => {
            //isV2supported(c.clone(), host).await;
            println!("ok1");
            let scopes:&[&str] = &["registry:catalog:*"];
            match c.authenticate(scopes).await {
                Ok(authc) => {
                    catalog(authc).await;
                    println!("ok2");
                    true
                },
                Err(_) => {
                    println!("err:auth");
                    false
                }
            }
        }
    }
}

async fn catalog(c: Client) {
    use futures_util::{pin_mut, stream, Stream, StreamExt};
    println!("hello");
    let s = c.get_catalog(Some(10));
    pin_mut!(s);
    while let Some(i) = s.next().await {
        match i {
            Ok(i) => println!("{}", i),
            Err(err) => println!("err:catalog"),
        }
    }
}

async fn list(c: Client) {
    
}

async fn isV2supported(c: Client, host: &str) -> bool {
    let span = span!(Level::TRACE, "my_span");
    let _enter = span.enter();
    match c.is_v2_supported().await {
        Err(err) => panic!("err"),
        Ok(supported) => {
            if !supported {
                println!("{} does NOT support v2", host)
            } else {
                println!("{} supports v2", host)
            };
            supported
        }
    }
}
