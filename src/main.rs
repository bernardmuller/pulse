use std::error::Error;
use std::io::Cursor;
// use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use axum::{ routing::post, Router,body::Bytes, 
// http::{ HeaderMap, }
};


#[derive(Debug, Deserialize, Serialize)]
pub struct MoodEntry {
    full_date: String,
    date: String,
    weekday: String,
    time: String,
    mood: String,
    activities: String,
    note_title: String,
    note: String,
}

pub fn parse_csv_string(csv_data: &str) -> Result<Vec<MoodEntry>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(Cursor::new(csv_data));
    
    let mut entries = Vec::new();
    for result in rdr.deserialize() {
        let record: MoodEntry = result?;
        entries.push(record);
    }
    
    Ok(entries)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/log", post(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(body: Bytes) {

    let body_str = String::from_utf8(body.to_vec()).unwrap();

    let entries = parse_csv_string(&body_str).unwrap();

    let first_entry = entries.first().unwrap();
    println!("{:?}: {:?}", first_entry.full_date, first_entry.mood);
}

