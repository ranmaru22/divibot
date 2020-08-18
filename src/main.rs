use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod config;
use config::Config;

mod modules;
use modules::dice;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("error sending message: {:?}", why)
    }
    Ok(())
}

#[command]
#[aliases("r")]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let dice_args = args
        .single::<String>()?
        .split(dice::is_delimiter)
        .map(|c| c.parse().unwrap_or_default())
        .collect();
    let random_rolls = match dice::roll(dice_args) {
        Some(result) => result,
        None => return Ok(()),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_rolls).await {
        println!("error sending message: {:?}", why)
    }

    Ok(())
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
