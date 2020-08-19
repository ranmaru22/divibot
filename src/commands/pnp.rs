use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

mod dice;
mod types;
use types::RollOptions;

#[command]
#[aliases("r")]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let dice_args = args
        .single::<String>()?
        .split(dice::is_delimiter)
        .map(|c| c.parse().unwrap_or_default())
        .collect();

    let mut opts = Vec::new();
    while !args.is_empty() {
        if let Ok(opt) = args.single::<String>() {
            match opt {
                opt if opt.contains("-e") => {
                    let arg = arg_parse(&opt, "-e");
                    opts.push(RollOptions::ExplodeOn(arg));
                }
                opt if opt.contains("-c") => {
                    let arg = arg_parse(&opt, "-c");
                    opts.push(RollOptions::CountSuccesses(arg));
                }
                _ => continue,
            }
        }
    }
    if opts.is_empty() {
        opts.push(RollOptions::Nothing);
    }

    let random_rolls = match dice::roll_dice(dice_args, opts) {
        Some(result) => result,
        None => return Ok(()),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_rolls).await {
        println!("error sending message: {:?}", why)
    }

    Ok(())
}

fn arg_parse(arg: &str, split_on: &str) -> u32 {
    *arg.split(split_on)
        .map(|c| c.parse().unwrap_or_default())
        .collect::<Vec<u32>>()
        .last()
        .unwrap_or(&0)
}
