use std::time::Duration;
use serenity::futures::stream::StreamExt;
use serenity::prelude::*;
use crate::commands::meme::BOT_ID;
use crate::reddit::post::{alt_reddit_post, sample_post};

use serenity::framework::standard::CommandResult;
use serenity::{model::prelude::Message, framework::standard::macros::command};
use tracing::log::info;


const PREV : char = '⬅';
const NEXT : char = '➡';
const GOOD : char = '👌';
const BAD  : char = '👎';
const COLLECTOR_DURATION : u64 = 60;



//#[command]
//#[description("new version of meme")]
//pub async fn koko(ctx: &Context, msg: &Message) -> CommandResult {
    //alt_reddit_post().await;
    //Ok(())
//}

#[command]
#[description("new version of meme")]
pub async fn momo(ctx: &Context, msg: &Message) -> CommandResult {
    let mut post = sample_post();
    if let Ok(new_post) = alt_reddit_post().await {
        post = new_post;
        info!("request ok! content:  {} ", post.title);
    } 
    let mut reply = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&post.title)
                .url(&post.url)
                .field("subreddit:", &post.subreddit, false)
                .image(&post.thumbnail)
        })
        .reference_message(msg)
    }).await?;
    add_reactions(ctx, &reply).await?;

    let mut vec = Vec::new();
    vec.push(post.clone());
    let mut meme_idx = 0;
    let collector = &mut reply
        .await_reactions(ctx)
        .removed(true)
        .timeout(Duration::from_secs(COLLECTOR_DURATION))
        .author_id(msg.author.id)
        .build();

    while let Some(reaction) = collector.next().await {
        let rct: String = reaction.as_inner_ref().clone().emoji.as_data().to_string();
        let mut meme_change = false;
        if rct == NEXT.to_string() {
            info!("NEXT POST");
            if let Ok(new_post) = alt_reddit_post().await {
                info!("request ok! content:  {} ", new_post.title);
                vec.push(new_post);
                meme_idx += 1;
                meme_change = true;
            }
        } else if rct == PREV.to_string() && meme_idx > 0 {
            info!("PREV POST");
            meme_idx -= 1;
            meme_change = true;
        } else {
            let namexd = reaction.as_inner_ref().user(&ctx.http).await?.name;
            info!("Unknown reaction :( {}", namexd);
            let _ = msg.channel_id.send_message(&ctx.http, |m| {m.content(format!("{} se la come doblada", namexd))}).await;
        }
        let post = vec.get(meme_idx).unwrap().clone();
        info!("final post:  {}", post.title);
        //reply.suppress_embeds(ctx).await?;
        if meme_change{
            reply.edit(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(post.title)
                        .url(post.url)
                        .field("subreddit:", post.subreddit, false)
                        .image(post.thumbnail)
                })
            }).await?;
        }
    };
    Ok(())
}

pub async fn add_reactions(ctx: &Context, msg: &Message) -> CommandResult{
    msg.react(ctx, PREV).await?;
    msg.react(ctx, GOOD).await?;
    msg.react(ctx, BAD).await?;
    msg.react(ctx, NEXT).await?;
    Ok(())
}

pub async fn delete_momo_reactions(ctx: &Context, msg: &Message) {
    info!("HANDLE MoMo REACTIONS");
    info!("msgXD {}",msg.content);
    let messages = match msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.after(msg.id).limit(5))
        .await
    {
        Ok(messages) => messages,
        Err(_) => return,
    };
    info!("msgs {}", messages.len());

    let reply = match messages.iter().find(|message| {
        message.author.id == BOT_ID
            && message.referenced_message.is_some()
            && message.referenced_message.as_ref().unwrap().id == msg.id
    }) {
        Some(message) => message,
        None => return,
    };
    info!("replyL {}", reply.reactions.len());
    //let _ = reply.delete_reactions(&ctx.http).await;
    let _ = reply.delete_reaction_emoji(&ctx.http, PREV).await;
    let _ = reply.delete_reaction_emoji(&ctx.http, NEXT).await;
    info!("replyL {}", reply.reactions.len());
}
