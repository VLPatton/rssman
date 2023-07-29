use crate::conven::{self, UrlCache};
use rss::Channel;

use super::UrlItems;

pub fn get_all_items(categories: &Vec<String>, channel: Channel) {
    let items = channel.into_items();
    for item in items {
        let mut is_in_categories: bool = categories.len() == 0;
        for wanted in categories {
            for cat in item.categories() {
                is_in_categories = is_in_categories || wanted == cat.name();
            }
        }
        if is_in_categories {
            println!(
                "{}",
                conven::feed_item_str(
                    item.title(),
                    item.description(),
                    item.author(),
                    item.link()
                )
            );
        }
    }
}

pub fn get_items_cache(
    cache: &mut UrlCache,
    categories: &Vec<String>,
    channel: Channel,
    url: &str,
) {
    let items = channel.into_items();
    for item in items {
        let id = item.guid().unwrap().value();
        let mut is_cached = false;
        let mut url_idx = 0usize;
        if cache.urls.len() != 0 {
            for url_check in 0..cache.urls.len() {
                if cache.urls[url_check].name == url {
                    url_idx = url_check;
                    break;
                }
            }
            for i in &cache.urls[url_idx].ids {
                is_cached = is_cached || i == id;
            }
        }
        if !is_cached {
            let mut is_in_categories: bool = categories.len() == 0;
            for wanted in categories {
                for cat in item.categories() {
                    is_in_categories = is_in_categories || wanted == cat.name();
                }
            }
            if is_in_categories {
                println!(
                    "{}",
                    conven::feed_item_str(
                        item.title(),
                        item.description(),
                        item.author(),
                        item.link()
                    )
                );
            }

            if cache.urls.len() != 0 {
                cache.urls[url_idx].name = url.to_string();
                cache.urls[url_idx].ids.push(id.to_string());
            } else {
                cache.urls.push(UrlItems {
                    name: url.to_string(),
                    ids: vec![id.to_string()],
                })
            }
        }
    }
}
