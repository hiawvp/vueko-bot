use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    //let app_id = env::var("APP_ID").expect("error getting APP_ID");
    //let public_key = env::var("PUBLIC_KEY").expect("error getting public_key");
    //let client_secret = env::var("CLIENT_SECRET").expect("error getting client_secret");
    //println!("got APP_ID: {app_id}");
    //println!("got PUBLIC_KEY: {public_key}");
    //println!("got CLIENT_SECRET: {client_secret}");

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    // no llega a este punto creo XD
    println!("vueko is up and running!");
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let content = &msg.content[..];
    let channel_id = msg.channel_id;
    println!("we got the message {content} at channel with id {channel_id}");
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
