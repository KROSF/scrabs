use super::repository::repository;
use crate::common::utils::check_msg;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[aliases("gh")]
fn github(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        check_msg(msg.channel_id.say(&ctx.http, "Args cannot be empty"));
        return Ok(());
    }

    let repo = args.message().to_string();

    let res = match repository(&repo) {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e);
            check_msg(msg.channel_id.say(&ctx.http, "Error with api"));
            return Ok(());
        }
    };

    println!("{:?}", res.data.unwrap().repository.unwrap().created_at);

    Ok(())
}
