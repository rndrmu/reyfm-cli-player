extern crate discord_rpc_client;
use discord_rpc_client::Client;

static mut COUNTER: i32 = 0;

pub async fn init_presence(upper: String, _lower: String) {
     // Create the client
     let mut drpc = Client::new(894291619623342090);

     // Start up the client connection, so that we can actually send and receive stuff

     // if counter is 0, then start presence as it is the first time
     unsafe {
        if COUNTER  == 0 {
            drpc.start();
            COUNTER += 1;
        }
     }

 
     // Set the activity
     drpc.set_activity(|act| 
        act.state("written in rust")
        .details(format!("Listening to #{}", upper.to_uppercase()))
        .assets(|assets| 
            assets.large_image("reyfm")
            .large_text("REYFM Client written in Rust")
        ))
         .expect("Failed to set activity");
 
}