use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
//use serenity::model::prelude::Embed;
use tracing::info;
//use serenity::Embed;

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
    title: String,
    url: Option<String>,
}

impl ChildrenData {
    pub fn content(&self) -> String {
        let url = self.url.clone().unwrap_or(String::from(""));
        format!("**{}**\n {}", self.title, url)
    }
    pub fn unpack(&self) -> (String, String){
        let url = self.url.clone().unwrap_or(String::from(""));
        (self.title.clone(), url)
    }
    //pub fn embed(&self) -> Embed {
        //let url = self.url.clone().unwrap_or(String::from(""));
        //Embed::{title: url}
        ////format!("**{}**\n {}", self.title, url)
    //}
}

// TODO: store options in ctx.data
pub async fn fetch_reddit_post() -> Result<ChildrenData, Box<dyn std::error::Error>> {
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
