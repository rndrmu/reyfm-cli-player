mod stuff;

use reqwest::Result;
use std::{env, process::Command, thread, time::Duration};




#[tokio::main]
async fn main() -> Result<()> {



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
    
    

    let mut child = Command::new("/usr/bin/mpv")
                        .arg(format!("{}", stream_url.unwrap()))
                        .arg("cache=no")
                        .arg("--really-quiet")
                        .arg("--no-config")
                        .spawn()
                        .expect("failed to start mpv! is it installed?");

    let _ecode = child.wait()
        .expect("failed to wait on child");
    let _no = child.stdout.as_mut().unwrap();


    Ok(())
}
