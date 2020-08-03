use self::inputs::SearchQueryInput;
use wasm_bindgen::prelude::*;
use atom_syndication::{Feed, Entry};
use std::error::Error;
use reqwest::get;


#[wasm_bindgen]
pub async fn search_puzzles(query_string: String) -> String {
  let search_query: SearchQueryInput = serde_json::from_str::<SearchQueryInput>(&query_string).expect("Unable to parse query");

  let channel = get_channel();
  let feed: Feed = channel.await.expect("Error getting channel");

  let entries = feed.entries.into_iter()
    .filter(|entry: &Entry| entry.title().contains(search_query.difficulty.as_str()) 
                        && entry.content()
                            .expect("Could not get content!")
                            .value()
                            .expect("Could not get content value")
                            .contains(search_query.player.as_str()))
    .collect::<Vec<Entry>>();

  if let Ok(resp) = serde_json::to_string(&entries) {
    resp
  } else {
    String::from("Could not find")
  }
}

async fn get_channel() -> Result<Feed, Box<dyn Error>> {
  let content = get("https://ditaktic.blogspot.com/feeds/posts/default").await?.bytes().await?;
  let feed = Feed::read_from(&content[..])?;
  Ok(feed)
}

mod inputs;
