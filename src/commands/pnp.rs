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

    let opts = if args.is_empty() {
        vec![RollOptions::Nothing]
    } else {
        create_opts(args)
    };

    let rolls = match dice::roll_dice(dice_args, opts) {
        Some(result) => result,
        None => return Ok(()),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &rolls).await {
        println!("error sending message: {:?}", why)
    }

    Ok(())
}

fn create_opts(mut args: Args) -> Vec<RollOptions> {
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
    opts
}

fn arg_parse(arg: &str, split_on: &str) -> u32 {
    *arg.split(split_on)
        .map(|c| c.parse().unwrap_or_default())
        .collect::<Vec<u32>>()
        .last()
        .unwrap_or(&0)
}
