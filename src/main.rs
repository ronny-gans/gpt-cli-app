use crate::services::make_connection;
use std::io::Write;
mod models;
mod services;
use std::io;
use colored::*;
use tokio;

#[tokio::main]
async fn main() {
    println!("{}","what i can help?".green());
    loop {
    let mut prompt = String::new();
    print!("{}","You: ".cyan());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut prompt)
        .expect("failed to get prompt");
    let prompt=prompt.trim();
    if prompt.is_empty() {
        continue;
    }
    match make_connection(prompt.to_string()).await {
        Ok(response) => {
            let content=&response.choices;
            for output in content {
                println!("{} {}","GPT Response: ".yellow(),
                output.message.content.yellow());
            }
            }
        Err(e) => eprintln!("Error {}",e)
        }
    }
}
