use thiserror;
use thiserror::Error;
use tokio;
use dotenv::dotenv;
use std::env;
use std::thread;


#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv().ok();

    discord::run(env::var("BOT_TOKEN")?).await?;

    /* let handle = thread::spawn(|| {
        loop {
            pixiv::find();
            discord::send();
        }
    }); */
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error : get env error")]
    VarError(#[from] std::env::VarError),
    #[error("Error : Addr parse error")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("Error : io error")]
    IoError(#[from] std::io::Error),
    #[error("Error : json error")]
    JsonError(#[from] serde_json::Error),
    #[error("Error : WS error")]
    WSSError(#[from] tokio_tungstenite::tungstenite::Error)
}

mod pixiv;
mod discord;