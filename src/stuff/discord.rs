extern crate discord_rpc_client;
use discord_rpc_client::Client;

static mut COUNTER: i32 = 0;

use std::{env, process::Command, thread, time::Duration};

use crate::NOW_PLAYING;

pub async fn init_presence(upper: String, lower: String) {
    // Create the client
    let mut drpc = Client::new(894291619623342090);

    // Start up the client connection, so that we can actually send and receive stuff

    // if counter is 0, then start presence as it is the first time
    drpc.start();

    unsafe {
        // Set the activity
        drpc.set_activity(|act| {
            act.state(lower.clone())
                .details(format!("Listening to #{}", upper.to_uppercase()))
                .assets(|assets| {
                    assets
                        .large_image("reyfm")
                        .large_text("REYFM Client written in Rust")
                })
        })
        .expect("Failed to set activity");
    }

    tokio::spawn(async move {
        loop {
            // wait for 5 secs
            thread::sleep(Duration::from_secs(15));
            // Set the activity
            update_presence(upper.clone(), lower.clone(), drpc.clone()).await;
        }
    });
}

async fn update_presence(upper: String, lower: String, mut client: Client) {
    unsafe {
        // Set the activity
        client
            .set_activity(|act| {
                act.state(NOW_PLAYING.clone())
                    .details(format!("Listening to #{}", upper.to_uppercase()))
                    .assets(|assets| {
                        assets
                            .large_image("reyfm")
                            .large_text("REYFM Client written in Rust")
                    })
            })
            .expect("Failed to set activity");
    }
}
