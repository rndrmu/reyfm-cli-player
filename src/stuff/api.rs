
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

use crate::stuff::discord;


static mut NOW_PLAYING: String = String::new();


pub async fn init_channel(channel_id: String) -> Option<String> {
    let chn = resolve_channel(&channel_id).await;
    
    if chn.is_none() {
        println!("Channel {} not found", channel_id);
        return None;
    }
    
    let request_url = format!("https://api.reyfm.de/v4/channel?chn={}", chn.unwrap());
    let response = reqwest::get(&request_url).await.unwrap().json::<serde_json::Value>().await.unwrap();
    let _channel_description = response["channel"]["description"].as_str().unwrap();
    let channel_name = response["channel"]["name"].as_str().unwrap();
    let _stream_url = response["channel"]["stream_urls"]["high"].as_str().unwrap();
    let song_title = response["channel"]["now"]["title"].as_str().unwrap();
    let song_artist = response["channel"]["now"]["artist"].as_str().unwrap();
    // get the stream url
    let stream_url = response["channel"]["stream_urls"]["high"].as_str().unwrap();
    discord::init_presence(channel_name.to_owned(), song_artist.to_owned()).await;

    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!(
        "[{}] -> [REYFM #{}] \n {} - {}", 
        datetime.format("%d/%m/%Y %T"), 
        channel_id, 
        song_artist, 
        song_title
    );

    unsafe {
        NOW_PLAYING = format!("{} - {}", song_artist, song_title);
    }
    
    Some(stream_url.to_owned())
}


pub async fn update_channel_info(channel_id: &String) {
    let chn = resolve_channel(channel_id.as_ref()).await;

    let request_url = format!("https://api.reyfm.de/v4/channel?chn={}", chn.unwrap());
    let response = reqwest::get(&request_url).await.unwrap().json::<serde_json::Value>().await.unwrap();
    let _channel_description = response["channel"]["description"].as_str().unwrap();
    let _channel_name = response["channel"]["name"].as_str().unwrap();
    let _stream_url = response["channel"]["stream_urls"]["high"].as_str().unwrap();
    let song_title = response["channel"]["now"]["title"].as_str().unwrap();
    let song_artist = response["channel"]["now"]["artist"].as_str().unwrap();
    // get the stream url

    unsafe {
        if NOW_PLAYING == format!("{} - {}", song_artist, song_title) {
            //println!("no change detected!");
            return;
        } else {
            // update 
            NOW_PLAYING = format!("{} - {}", song_artist, song_title);
        }
    }



    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!(
        "[{}] -> [REYFM #{}] \n {} - {}", 
        datetime.format("%d/%m/%Y %T"), 
        channel_id, 
        song_artist, 
        song_title
    );
    //discord::init_presence(song_title.to_owned(), song_artist.to_owned()).await;
}

async fn resolve_channel(channel_id: &str) -> Option<i32> {
    let channel_number = match channel_id.to_lowercase().as_str() {
        "original" => 1,
        "nightlife" => 2,
        "raproyal" => 3,
        "usrap" => 4,
        "hitsonly" => 5,
        "gaming" => 6,
        "houseparty" => 7,
        "chillout" => 8,
        "lofi" => 9,
        "oldschool" => 10,
        "mashup" => 11,
        "charts" => 12,
        "partyhard" => 13,
        "bass" => 14,
        "kpop" => 15,
        _ => {
            return None;
        }
    };

    return Some(channel_number);
}