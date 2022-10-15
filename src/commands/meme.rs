//use futures::join;
use crate::reddit::post::fetch_reddit_post;
use crate::ReactionTypeContainer;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use tracing::log::info;

pub const BOT_ID: u64 = 1010296921274974379;
const HALAL_MSG: &str = "SI TE GUSTO EL MEME SUSCRIBETE";
const HARAM_MSG: &str = "NO TE GUSTO EL MEME AHHH?";
const HARAM_MSG_P2: &str = "AKI TE VA OTRO\n https://media.discordapp.net/attachments/1010298574241800302/1012487429829169282/FZkBXMiXkAAQLxm.jpeg.jpg?width=556&height=685 ";

#[command]
pub async fn emojix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("EMOJIX");
    let emoji_name = args.single::<String>()?;
    info!("args received: {}", emoji_name);
    let response;
    if let Some(emoji) = get_emoji_by_name(ctx, msg, &emoji_name).await {
        response = format!("{}", emoji);
    } else {
        response = String::from("xd");
    }
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

#[command]
pub async fn react(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("REACT");
    let subreddit = args.single::<String>()?;
    info!("args received: {}", subreddit);
    let author = &msg.author.name;
    info!("command received from {:?}", author);
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

async fn get_emoji_by_name(ctx: &Context, msg: &Message, name: &str) -> Option<Emoji> {
    info!("get emoji by name");
    //let guild_id: u64 = 955133006627082240;
    if let Some(guild_id) = msg.guild_id {
        info!("guild id: {}", guild_id);
        if let Some(emojis) = ctx.cache.guild_field(guild_id, |v| v.emojis.clone()) {
            info!("emojis: {}", emojis.len());
            if let Some(found_emoji) = emojis.values().find(|v| v.name == name) {
                info!("found_emoji id: {}", found_emoji.id);
                return Some(found_emoji.clone());
            }
        }
    }
    None
}

// TODO: store emojis in ctx.data
async fn get_emoji(ctx: &Context, name: &str) -> Option<ReactionType> {
    info!("get emoji!");
    //let guild_id: u64 = 955133006627082240;
    let guild_id: u64 = 555183947244634112;
    let emoji_names: HashMap<String, u64> = HashMap::from([
        (String::from("haram"), 953465443312623706u64),
        (String::from("halal"), 953465392607690752u64),
        (String::from("kaos"), 957368542423052288u64),
    ]);
    let emoji_id = match emoji_names.get(name) {
        Some(emoji_id) => *emoji_id,
        None => return None,
    };
    match ctx
        .cache
        .guild(guild_id)
        .unwrap()
        .emoji(&ctx.http, emoji_id.into())
        .await
    {
        Ok(v) => Some(v.into()),
        Err(_) => None,
    }
}

#[command]
#[description("get them good memes")]
//pub async fn meme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
pub async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    let response: String;
    if let Ok(res) = fetch_reddit_post().await {
        response = res.content();
        info!("request ok! content:  {}", response);
    } else {
        response = String::from(":TrollFace:");
    }

    let reply = msg.reply(&ctx.http, response).await?;
    let halal: ReactionType = get_emoji(&ctx, "halal")
        .await
        .unwrap_or_else(|| '👌'.into());
    let haram: ReactionType = get_emoji(&ctx, "haram")
        .await
        .unwrap_or_else(|| '👎'.into());
    reply.react(ctx, halal).await?;
    reply.react(ctx, haram).await?;
    //reply.content\c

    //if let Some(reaction) = &reply.await_reaction(&ctx).timeout(Duration::from_secs(10)).await{

    //}
    //join!(reply.react(ctx, halal), reply.react(ctx, haram));
    Ok(())
}

//#[command]
//#[description("new version of meme")]
////pub async fn meme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
//pub async fn momo(ctx: &Context, msg: &Message) -> CommandResult {
    //let response: String;
    //let prev = '⬅';
    //let next = '➡';
    //let mut meme_idx = 0;
    //let mut vec = Vec::new();

    //if let Ok(res) = fetch_reddit_post().await {
        //response = res.content();
        //info!("request ok! content:  {}", response);
    //} else {
        //response = String::from(":TrollFace:");
    //}
    //vec.push(response.clone());
    //let mut reply = msg.reply(&ctx.http, response).await?;
    //let halal: ReactionType = get_emoji(&ctx, "halal")
        //.await
        //.unwrap_or_else(|| '👌'.into());
    //let haram: ReactionType = get_emoji(&ctx, "haram")
        //.await
        //.unwrap_or_else(|| '👎'.into());
    ////reply.react(&ctx, '✅').await?;
    //reply.react(ctx, prev).await?;
    //reply.react(ctx, halal).await?;
    //reply.react(ctx, haram).await?;
    //reply.react(ctx, next).await?;

    //let collector = &mut reply
        //.await_reactions(ctx)
        //.removed(true)
        //.timeout(Duration::from_secs(60))
        //.author_id(msg.author.id)
        //.build();

    ////let http = &ctx.http;
    ////
    //while let Some(reaction) = collector.next().await {
        //let rct: String = reaction.as_inner_ref().clone().emoji.as_data().to_string();
        //let response;
        //info!("reaction received {}", rct);
        //if rct == next.to_string() {
            //if let Ok(res) = fetch_reddit_post().await {
                //response = res.content();
                ////response = res.url.unwrap();
                //info!("request ok! content:  {}", response);
            //} else {
                //response = String::from(":TrollFace:");
            //}
            //vec.push(response.clone());
            //meme_idx += 1;
        //} else if rct == prev.to_string() && meme_idx > 0 {
            //meme_idx -= 1;
            //response = vec[meme_idx].clone();
        //} else {
            //response = String::from(":TrollFace:");
        //}
        //info!("meme_idx {}", meme_idx);
        ////reply.suppress_embeds(ctx).await?;
        //reply.edit(ctx, |m| m.content(response)).await?;
    //}
    //Ok(())
//}



pub async fn handle_meme_reactions(ctx: &Context, msg: &Message) {
    info!("HANDLE MEME REACTIONS");
    let _ = tokio::time::sleep(Duration::from_secs(10)).await;
    let messages = match msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.after(msg.id).limit(5))
        .await
    {
        Ok(messages) => messages,
        Err(_) => return,
    };

    let reply = match messages.iter().find(|message| {
        message.author.id == BOT_ID
            && message.referenced_message.is_some()
            && message.referenced_message.as_ref().unwrap().id == msg.id
    }) {
        Some(message) => message,
        None => return,
    };

    let (haram, halal) = {
        let data_read = ctx.data.read().await;
        let reactions_lock = data_read
            .get::<ReactionTypeContainer>()
            .expect("expected reactiontypes in mappu")
            .clone();

        let stored_reactions = reactions_lock.read().await;
        (
            stored_reactions.get("haram").unwrap().clone(),
            stored_reactions.get("halal").unwrap().clone(),
        )
    };
    let positive_response = format!("{} {}", HALAL_MSG, halal);
    let negative_response = format!("{} {}\n{}", HARAM_MSG, haram, HARAM_MSG_P2);
    reply_to_reactions(ctx, reply, halal, &positive_response).await;
    reply_to_reactions(ctx, reply, haram, &negative_response).await;
}

async fn reply_to_reactions(
    ctx: &Context,
    meme_msg: &Message,
    emoji: ReactionType,
    response: &str,
) {
    let reactionsxd = ctx
        .http
        .get_reaction_users(
            meme_msg.channel_id.into(),
            meme_msg.id.into(),
            &emoji,
            10,
            None,
        )
        .await;

    match reactionsxd {
        Ok(users) => {
            for user in users {
                if user.id == BOT_ID {
                    continue;
                }
                info!("dming user with tag: {}", user.tag());
                let _ = user
                    .direct_message(&ctx.http, |m| m.content(response))
                    .await;
            }
        }
        Err(e) => info!("error! {}", e),
    };
}
