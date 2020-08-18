use serenity::{
    async_trait,
    framework::standard::{macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

mod config;
use config::Config;

mod commands;
use commands::{meta::*, pnp::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[group]
#[commands(ping, roll)]
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
