use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
//use serenity::model::prelude::Embed;
use tracing::info;
//use serenity::Embed;
//
use roux::Subreddit;


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
pub struct ChildrenData {
    //subreddit: String,
    pub title: String,
    pub subreddit: String,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    domain: String,
    stickied: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemePost {
    pub title: String,
    pub subreddit: String,
    pub thumbnail: String,
    pub url: String,
}

pub fn sample_post() -> MemePost{
    MemePost{
        title : String::from(":TrollFace:"),
        url : String::from(""),
        thumbnail :String::from(""),
        subreddit : String::from(""),
    } 
}

impl ChildrenData {
    pub fn content(&self) -> String {
        let url = self.url.clone().unwrap_or(String::from(""));
        format!("**{}**\n {}", self.title, url)
    }
    pub fn to_meme(&self) -> MemePost{
        let url = self.url.clone().unwrap_or(String::from(""));
        let thumbnail = match self.domain.as_str(){
            "v.redd.it" => self.thumbnail.clone().unwrap_or(String::from("")),
            _ => url.clone(),
        };
        MemePost { title: self.title.clone(), subreddit: self.subreddit.clone(), thumbnail, url}
    }
}

pub fn get_random_sr_name() -> &'static str{
    let options = vec![
        "okbuddybaka",
        "okbuddyretard",
        "okbuddyphd",
        "yo_ctm",
        "dankgentina",
        "196",
        "2hujerk",
    ];
    options.choose(&mut rand::thread_rng()).unwrap().clone()
}

pub async fn alt_reddit_post() -> Result<MemePost, Box<dyn std::error::Error>>{
    let subr = get_random_sr_name();
    let subreddit = Subreddit::new(subr);
    let hot = subreddit.hot(25, None).await;
    let hot_string = serde_json::to_string_pretty(&hot.unwrap()).unwrap();
    match serde_json::from_str(&hot_string) {
        Ok::<RedditResponse, _>(v) => {
            let post : ChildrenData;
            loop {
                let selected_post = v.data.children.choose(&mut rand::thread_rng()).unwrap();
                if selected_post.data.stickied {
                    info!("FOUND STICKY!");
                } else {
                    info!("valid post");
                    post = selected_post.data.clone();
                    {break;}
                }
            }
            println!("{}", post.content());
            Ok(post.to_meme())
        },
        Err(e) => {
            info!("ERROR! {}", e);
            Err(Box::new(e))
        }
    }
}

// TODO: store options in ctx.data
pub async fn fetch_reddit_post() -> Result<ChildrenData, Box<dyn std::error::Error>> {
    let subr = get_random_sr_name();
    let url = format!("https://www.reddit.com/r/{}/random.json", subr);
    info!("url: {:#?}", url);
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
