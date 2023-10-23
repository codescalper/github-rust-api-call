use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;
use std::io;
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut owner = String::new();
    println!("Please enter the name of the user: ");
    io::stdin()
        .read_line(&mut owner)
        .expect("Failed to read line");

    let mut repo = String::new();
    println!("Please enter the name of the user's repo: ");
    io::stdin()
        .read_line(&mut repo)
        .expect("Failed to read line");

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",);
    println!("{:#?}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust github-api call project")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}
