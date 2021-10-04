mod stuff;

use dotenv;
use reqwest::Result;
use std::{env, process::Command, thread, time::Duration};

use stuff::discord;

pub static mut NOW_PLAYING: String = String::new();
pub static mut CURRENT_CHANNEL: String = String::new();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    let mpv_path = env::var("MPV_PATH").expect("MPV_PATH not set");

    let channel = env::args().nth(1).unwrap_or("".to_string());
    let chn = channel.clone();
    let stream_url = stuff::api::init_channel(channel).await;

    if stream_url.is_none() {
        println!("Channel not found");
        return Ok(());
    }

    // spawn a thread to keep updating the metadata
    tokio::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(15));
            stuff::api::update_channel_info(&chn.clone()).await;
        }
    });

    // spawn a thread to keep updating the now playing

    let mut child = Command::new(mpv_path)
        .arg(format!("{}", stream_url.unwrap()))
        .arg("cache=no")
        .arg("--really-quiet")
        .arg("--no-config")
        .spawn()
        .expect("failed to start mpv! is it installed?");

    let _ecode = child.wait().expect("failed to wait on child");
    let _no = child.stdout.as_mut().unwrap();

    Ok(())
}
