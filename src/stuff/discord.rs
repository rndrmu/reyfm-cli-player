extern crate discord_rpc_client;
use discord_rpc_client::Client;

static mut COUNTER: i32 = 0;

use std::{env, process::Command, thread, time::Duration};
use chrono::{self, Datelike};

use crate::{
    NOW_PLAYING,
    IS_LIVE_SHOW
};

pub async fn init_presence(upper: String, lower: String) {
    // Create the client
    let mut drpc = Client::new(894291619623342090);

    // Start up the client connection, so that we can actually send and receive stuff

    // if counter is 0, then start presence as it is the first time
    drpc.start();

    unsafe {

        let mut bigimg = "reyfm";
        if IS_LIVE_SHOW {
            // check if today is monday
            let today = chrono::Local::today();
            if today.weekday() == chrono::Weekday::Mon {
                bigimg = "live_1";
            } else if today.weekday() == chrono::Weekday::Fri {
                bigimg = "live_2";
            }
        }
        // Set the activity
        drpc.set_activity(|act| {
            act.state(lower.clone())
                .details(format!("Listening to #{}", upper.to_uppercase()))
                .assets(|assets| {
                    assets
                        .large_image(bigimg.to_owned())
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

        let mut bigimg = "reyfm";
        if IS_LIVE_SHOW {
            // check if today is monday
            let today = chrono::Local::today();
            if today.weekday() == chrono::Weekday::Mon {
                bigimg = "live_1";
            } else if today.weekday() == chrono::Weekday::Fri {
                bigimg = "live_2";
            }
        }

        client
            .set_activity(|act| {
                act.state(NOW_PLAYING.clone())
                    .details(format!("Listening to #{}", upper.to_uppercase()))
                    .assets(|assets| {
                        assets
                            .large_image(bigimg.to_owned())
                            .large_text("REYFM Client written in Rust")
                    })
                    
            })
            .expect("Failed to set activity");
    }
}
