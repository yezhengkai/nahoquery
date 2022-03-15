#![allow(unused)]

use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = nahoquery::get_vars("2022-03-10 00:00", "2022-03-11 00:00", "").await;
    match text {
        Ok(text) => {
            println!("{}", text);
        }
        Err(e) => println!("{}", e),
    }

    let text = nahoquery::download(
        "2022-03-10 00:00",
        "2022-03-10 00:10",
        vec!["Rain01", "CWB_Humidity"],
    )
    .await;
    match text {
        Ok(text) => {
            println!("{}", text);
        }
        Err(e) => println!("{}", e),
    }
    Ok(())
}
