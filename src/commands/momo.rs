use std::time::Duration;
use serenity::futures::stream::StreamExt;
use serenity::prelude::*;
use crate::commands::meme::BOT_ID;
use crate::reddit::post::alt_reddit_post;

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
    let title: String;
    let url: String;
    let thumbnail: String;
    let subreddit: String;
    let mut meme_idx = 0;
    let mut vec = Vec::new();

    if let Ok(res) = alt_reddit_post().await {
        (title, subreddit, thumbnail, url) = res.unpack();
        info!("request ok! content:  {} ", title);
    } else {
        title = String::from(":TrollFace:");
        url = String::from("");
        thumbnail = String::from("");
        subreddit = String::from("");
    }

    vec.push((title.clone(), url.clone()));
    let mut reply = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(title)
                .url(url)
                .field("subreddit:", subreddit, false)
                .image(thumbnail)
        })
        .reference_message(msg)
    }).await?;
    reply.react(ctx, PREV).await?;
    reply.react(ctx, GOOD).await?;
    reply.react(ctx, BAD).await?;
    reply.react(ctx, NEXT).await?;

    let collector = &mut reply
        .await_reactions(ctx)
        .removed(true)
        .timeout(Duration::from_secs(COLLECTOR_DURATION))
        .author_id(msg.author.id)
        .build();


    while let Some(reaction) = collector.next().await {
        let rct: String = reaction.as_inner_ref().clone().emoji.as_data().to_string();
        info!("reaction received {}", rct);
        let mut title = String::from(":TrollFace:");
        let mut url = String::from("");
        let mut thumbnail = String::from("");
        let mut subreddit = String::from("");
        let mut meme_change = false;
        if rct == NEXT.to_string() {
            if let Ok(res) = alt_reddit_post().await {
                (title, subreddit, thumbnail, url) = res.unpack();
                info!("request ok! content:  {} ", title);
                vec.push((title.clone(), url.clone()));
                meme_idx += 1;
                meme_change = true;
            }
        } else if rct == PREV.to_string() && meme_idx > 0 {
            meme_idx -= 1;
            meme_change = true;
            (title, url) = vec[meme_idx].clone();
        } 
        info!("meme_idx {}", meme_idx);
        //reply.suppress_embeds(ctx).await?;
        if meme_change{
            info!("gonna edit embed with:  {} {} ", title, url);
            reply.edit(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(title)
                        .url(url)
                        .field("subreddit:", subreddit, false)
                        .image(thumbnail)
                })
            }).await?;
        }
    }
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
