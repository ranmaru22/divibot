use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

mod dice;
use dice::RollOptions;

#[command]
#[aliases("r")]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let dice_args = args
        .single::<String>()?
        .split(dice::is_delimiter)
        .map(|c| c.parse().unwrap_or_default())
        .collect();
    let opts = RollOptions::Nothing;
    let random_rolls = match dice::roll_dice(dice_args, opts) {
        Some(result) => result,
        None => return Ok(()),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_rolls).await {
        println!("error sending message: {:?}", why)
    }

    Ok(())
}
