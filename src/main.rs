use dotenv::dotenv;
// ----------------------------------
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::all::VoiceState;
use serenity::all::ChannelType;
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
        
        if new.channel_id.is_some() && new.channel_id.unwrap().name(&ctx.http).await.unwrap() == "Join to Create"{

            let guild_id = new.guild_id.unwrap();

            let channel = guild_id.create_channel(&ctx.http, |c| c.name("Testies")
                .kind(ChannelType::Voice).bitrate(64000).user_limit(10))
                .await.unwrap();

            let channel_id = channel.id;

            guild_id.move_member(&ctx.http, new.user_id, channel_id).await.unwrap();
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
