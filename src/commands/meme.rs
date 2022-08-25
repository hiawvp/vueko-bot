//use futures::join;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use tracing::log::info;

//#[derive(Hash, Eq, PartialEq, Debug)]
//struct EmojiMap {
//name: String,
//id: u64,
//}

//impl EmojiMap {
///// Creates a new Viking.
//fn new(name: &str, id: &u64) -> EmojiMap {
//EmojiMap {
//name: name.to_string(),
//id: id.clone(),
//}
//}
//}

#[derive(Serialize, Deserialize)]
struct RedditResponse {
    data: Data,
    kind: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    //after: Option<String>,
    //dist: i32,
    modhash: Option<String>,
    //geo_filter: String,
    children: Vec<Child>,
}

#[derive(Serialize, Deserialize)]
struct Child {
    kind: String,
    data: ChildrenData,
    //data: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChildrenData {
    //subreddit: String,
    title: String,
    url: Option<String>,
}

impl ChildrenData {
    fn content(&self) -> String {
        let url = self.url.clone().unwrap_or(String::from(""));
        format!("**{}**\n {}", self.title, url)
    }
}

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

// TODO: store options in ctx.data
async fn fetch_reddit_post() -> Result<ChildrenData, Box<dyn std::error::Error>> {
    let options = vec![
        "okbuddybaka",
        "okbuddyretard",
        "okbuddyphd",
        "yo_ctm",
        "dankgentina",
        "196",
    ];
    let subr = options.choose(&mut rand::thread_rng()).unwrap();
    let url = format!("https://www.reddit.com/r/{}/random.json", subr);
    println!("url: {:#?}", url);
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    let first = match response.get(0) {
        Some(v) => v,
        None => &response,
    };
    match serde_json::from_value(first.clone()) {
        Ok::<RedditResponse, _>(r) => {
            info!("fetch reddit post ok");
            Ok(r.data.children[0].data.clone())
        }
        Err(e) => {
            info!("Error! {}", e);
            Err(Box::new(e))
        }
    }
}

#[command]
#[description("get them good memes")]
pub async fn meme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
