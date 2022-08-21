use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::log::info;

#[command]
pub async fn meme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("MEME");
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;
    info!("args received: {},  {}", one, two);

    let product = one * two;

    msg.channel_id.say(&ctx.http, product).await?;

    Ok(())
}

#[command]
pub async fn square(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("SQUARE");
    let val = args.single::<f64>()?;
    info!("args received: {}", val);

    let square = val * val;

    msg.channel_id.say(&ctx.http, square).await?;

    Ok(())
}

fn get_emoji(ctx: &Context, msg: &Message) -> Option<Emoji> {
    info!("get emoji!");
    if let Some(guild_id) = msg.guild_id {
        info!("guild id: {}", guild_id);
        if let Some(emojis) = ctx.cache.guild_field(guild_id, |v| v.emojis.clone()) {
            info!("emojis: {}", emojis.len());
            if let Some(kaos) = emojis.values().find(|v| v.name == "kaos") {
                info!("kaos id: {}", kaos.id);
                return Some(kaos.clone());
            }
        }
    }
    None
}

#[command]
pub async fn react(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("REACT");
    let subreddit = args.single::<String>()?;
    info!("args received: {}", subreddit);
    let author = &msg.author.name;
    info!("command received from {:?}", author);

    let reply = msg.channel_id.say(&ctx.http, subreddit).await?;
    if let Some(emoji) = get_emoji(&ctx, &msg) {
        reply.react(ctx, emoji).await?;
    } else {
        reply.react(ctx, '👌').await?;
    }
    Ok(())
}
