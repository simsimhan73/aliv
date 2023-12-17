use thiserror;
use thiserror::Error;
use tokio;
use dotenv::dotenv;
use std::env;
use std::thread;


#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv().ok();

    discord::run(env::var("BOT_TOKEN")?)?;

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
    #[error("error : get env error")]
    VarError(#[from] std::env::VarError),
    #[error("error : Addr parse error")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("error : io error")]
    IoError(#[from] std::io::Error),
    #[error("error : json error")]
    JsonError(#[from] serde_json::Error),
    #[error("")]
}

mod pixiv;
mod discord;