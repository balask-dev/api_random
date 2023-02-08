use serde::{Serialize,Deserialize,Debug};
use reqwest::header::USER_AGENT;
use reqwest::Error;
use std::env;

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main()-> std::io::Result<(),Error>{
    env::set_var("RUST_BACKTRACE", "1");
    let res = format!("https://jsonplaceholder.typicode.com/todos");
    println!("{}",res);
    let client = reqwest::Client::new();
    let cli = client.get(&res).header(USER_AGENT,"rustwebapp").send().await?;
    let result:Vec<User> = cli.json().await?;
    println!("{:?}",result);
    Ok(())
}
