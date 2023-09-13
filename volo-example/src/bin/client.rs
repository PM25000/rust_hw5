use lazy_static::lazy_static;
use pilota::FastStr;
use std::{
    io::{self, BufRead, Write},
    net::SocketAddr,
};
use volo_example::LogLayer;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:10818".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            // .layer_outer(LogLayer)
            .address(addr)
            .build()
    };
}

async fn get_item(key: FastStr) -> volo_gen::volo::example::GetItemResponse {
    let req = volo_gen::volo::example::GetItemRequest { key };
    let resp = CLIENT.get_item(req).await;
    match resp {
        Ok(info) => info,
        Err(e) => {
            tracing::error!("{:?}", e);
            Default::default()
        }
    }
}

async fn set_item(key: FastStr, value: FastStr) -> volo_gen::volo::example::SetItemResponse {
    let req = volo_gen::volo::example::SetItemRequest {
        kv: {
            let mut kv = volo_gen::volo::example::Kv::default();
            kv.key = key;
            kv.value = value;
            kv
        },
    };
    let resp = CLIENT.set_item(req).await;
    match resp {
        Ok(info) => info,
        Err(e) => {
            tracing::error!("{:?}", e);
            Default::default()
        }
    }
}

async fn delete_item(keys: Vec<FastStr>) -> volo_gen::volo::example::DeleteItemResponse {
    let req = volo_gen::volo::example::DeleteItemRequest { keys };
    let resp = CLIENT.delete_item(req).await;
    match resp {
        Ok(info) => info,
        Err(e) => {
            tracing::error!("{:?}", e);
            Default::default()
        }
    }
}

async fn ping(msg: Option<String>) -> volo_gen::volo::example::PingResponse {
    let req = volo_gen::volo::example::PingRequest {
        message: msg.map(|s| FastStr::from(s)),
    };
    let resp = CLIENT.ping(req).await;
    match resp {
        Ok(info) => info,
        Err(e) => {
            tracing::error!("{:?}", e);
            Default::default()
        }
    }
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut args = input.split_whitespace();
        let cmd = args.next().unwrap();
        let args = args.collect::<Vec<_>>();

        match cmd {
            "get" => {
                let key = args[0];
                let resp = get_item(String::from(key).into()).await;
                println!("{:?}", resp);
            }
            "set" => {
                let key = args[0];
                let value = args[1];
                let resp = set_item(String::from(key).into(), String::from(value).into()).await;
                println!("{:?}", resp);
            }
            "delete" => {
                let keys = args.iter().map(|s| String::from(*s).into()).collect();
                let resp = delete_item(keys).await;
                println!("{:?}", resp);
            }
            "ping" => {
                let msg = args.join(" ");
                let resp = if args.is_empty() {
                    ping(None).await
                } else {
                    ping(Some(msg)).await
                };
                println!("{:?}", resp);
            }
            "exit" => {
                break;
            }
            _ => {
                println!("unknown command: {}", cmd);
            }
        }
    }
}
