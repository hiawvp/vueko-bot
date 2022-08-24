use std::collections::HashMap;
use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = Arc<RwLock<HashMap<String, u64>>>;
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

//#[command]
//async fn help(ctx: &Context, msg: &Message) -> CommandResult {
//let content = &msg.content[..];
//let reply = format!("hey now, you are a rockstar {content}");
//msg.reply(&ctx.http, reply).await?;

//Ok(())
//}

/// Usage: `~command_usage <command_name>`
/// Example: `~command_usage ping`
#[command]
pub async fn command_usage(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = match args.single_quoted::<String>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(ctx, "I require an argument to run this command.")
                .await?;
            return Ok(());
        }
    };
    //ctx.cache.commands
    // Yet again, we want to keep the locks open for the least time possible.
    let amount = {
        // Since we only want to read the data and not write to it, we open it in read mode,
        // and since this is open in read mode, it means that there can be multiple locks open at
        // the same time, and as mentioned earlier, it's heavily recommended that you only open
        // the data lock in read mode, as it will avoid a lot of possible deadlocks.
        let data_read = ctx.data.read().await;

        // Then we obtain the value we need from data, in this case, we want the command counter.
        // The returned value from get() is an Arc, so the reference will be cloned, rather than
        // the data.
        let command_counter_lock = data_read
            .get::<CommandCounter>()
            .expect("Expected CommandCounter in TypeMap.")
            .clone();

        let command_counter = command_counter_lock.read().await;
        // And we return a usable value from it.
        // This time, the value is not Arc, so the data will be cloned.
        command_counter.get(&command_name).map_or(0, |x| *x)
    };

    if amount == 0 {
        msg.reply(
            ctx,
            format!("The command `{}` has not yet been used.", command_name),
        )
        .await?;
    } else {
        msg.reply(
            ctx,
            format!(
                "The command `{}` has been used {} time/s this session!",
                command_name, amount
            ),
        )
        .await?;
    }

    Ok(())
}
