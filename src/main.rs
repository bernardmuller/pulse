use std::error::Error;
use std::io::Cursor;
use chrono::{Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use axum::{
    body::Bytes, 
    http::HeaderMap, 
    routing::post, 
    Router,
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

async fn handler(
    headers: HeaderMap,
    body: Bytes,
)  {
    let authenticate = headers.get("x-authenticate");
    if authenticate.is_none() || authenticate.unwrap().to_str().unwrap() != "daylio" {
        println!("unauthorized");
        return
    }
    
    let body_str = match String::from_utf8(body.to_vec()) {
        Ok(str) => str,
        Err(_) => 
        {
            println!("Invalid UTF-8 data");
            return
        }
    };
    
    let entries = match parse_csv_string(&body_str) {
        Ok(entries) => entries,
        Err(_) =>{
            println!("Invalid CSV data");
            return
        }
    };

    let missing_entries = get_missing_entry_dates(&entries);

    println!("{:?}", missing_entries);
}

fn get_missing_entry_dates(entries: &Vec<MoodEntry>) -> Vec<NaiveDate> {
    let today = Local::now().date_naive();
    let latest_entry = NaiveDate::parse_from_str(&entries[0].full_date, "%Y-%m-%d").unwrap();
    let outstanding_entries = (today - latest_entry).num_days();

    if outstanding_entries == 0 {
        return Vec::new();
    }

    let mut missing_entries = Vec::new();
    for i in 0..outstanding_entries {
        let date = today.checked_sub_days(Days::new(i as u64));
        if date.is_some() {
            missing_entries.push(date.unwrap());
        } else {
            println!("Error: date is None");
            return Vec::new();
        }
    }

    missing_entries
}