use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use core::time;
use std::{env, thread};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    //let client_uri = "mongodb://adminuser:password123@localhost:32000/?retryWrites=true&w=majority";
    let client_uri = "mongodb://adminuser:password123@mongo-nodeport-svc:27017/?retryWrites=true&w=majority";

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    thread::sleep(time::Duration::from_millis(10 * 1000));

    Ok(())
}
