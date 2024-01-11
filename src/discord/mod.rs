use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Number, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::Url;

use crate::Error;


const GATEWAY_URL : &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub async fn run(token : String) -> Result<(), Error>{

    let json = make_indentify_content(token);

    let (mut stream , res)= connect_async(Url::parse(GATEWAY_URL)
        .expect("Failed to parsing URL")).await.expect("Failed to connect");

    stream.send(Message::from(json.to_string())).await.expect("Failed to Sending message");

    if let Some(msg) = stream.next().await {
        let msg = json!(msg?.to_string());
        println!("{}",msg);
        if let Some(Value::Number(op)) = msg.get("op") {
            if op {
                let interval = msg.get("d").unwrap().get("heartbeat_interval").unwrap().as_u64();
                println!("{:?}", interval);
            }
        }

    }



    Ok(())
}

fn make_indentify_content(token : String) -> serde_json::Value {
    let mut json =json!(
        {
            "op" : 2,
            "d" : {
            "token" : token,
            "intent" : 16384,
            "properties": {
                "os": "windows",
                "browser": "my_library",
                "device": "my_library"
              }
            }
        }
    );

    return json;
}
