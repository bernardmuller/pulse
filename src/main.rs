pub mod url_builder;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i32,
    userId: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Sending request to JSONPlaceholder API...");

    // Create a client
    let client = reqwest::Client::new();

    // Send a GET request
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Request successful!");

        // Parse the JSON response
        let post: Post = response.json().await?;

        // Print the post
        println!("Post ID: {}", post.id);
        println!("User ID: {}", post.userId);
        println!("Title: {}", post.title);
        println!("Body: {}", post.body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
