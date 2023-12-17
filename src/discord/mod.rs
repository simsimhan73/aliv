use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::io::{Write, Read};
use serde_json::{json};

use crate::Error;


const GATEWAY_URL : &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub async fn run(token : String) -> Result<(), Error>{

    let json = make_indentify_content(token);

    let mut stream = connect_async(GATEWAY_URL).await?;
    
    stream.write(json.to_string().as_bytes())?;

    let mut buf : String = "".to_string();

    stream.read_to_string(&mut buf)?;

    println!("{:?}", buf);

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
                "os": "linux",
                "browser": "my_library",
                "device": "my_library"
              }
            }
        }
    );

    return json;
}
