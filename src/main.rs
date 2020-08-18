use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod config;
use config::Config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
        println!("error sending message: {:?}", why)
    }
    Ok(())
}

#[command]
async fn foo(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "bar").await {
        println!("error sending message: {:?}", why)
    }
    Ok(())
}

#[group]
#[commands(ping, foo)]
struct Public;

#[tokio::main]
async fn main() {
    let config = Config::new();

    let framework = StandardFramework::new()
        .configure(|cfg| cfg.with_whitespace(true).prefix(config.prefix()))
        .group(&PUBLIC_GROUP);

    let mut client = Client::new(config.token())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
