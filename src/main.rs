use dotenv::dotenv;
// ----------------------------------
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::all::VoiceState;
// use serenity::all::GuildId;
// ----------------------------------

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content.contains("quack") {

            if let Err(why) = msg.channel_id.say(&ctx.http, "Quack!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState,) {
        
        // if new channel is not none and new channel is equal to a channel
        if new.channel_id.is_some() && new.channel_id.unwrap().name(&ctx.http).await.unwrap() == "Join to Create"{
            if let Err(why) = new.channel_id.unwrap().say(&ctx.http, "Quack!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

}


async fn run_bot() {
    dotenv().ok();
    let token = std::env::var("TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::all();

    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler).await
        .expect("Err creating client");
    
    println!("Bot created");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}


#[tokio::main]
async fn main() {
    println!("Starting up...");

    run_bot().await;
}
