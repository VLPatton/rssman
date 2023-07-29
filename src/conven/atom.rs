use crate::conven::{self, UrlCache};
use atom_syndication as atom;
use std::{error::Error, str::FromStr};

use super::UrlItems;

pub fn get_all_items(
    categories: &Vec<String>,
    xml: &str,
) -> Result<(), Box<dyn Error>> {
    let feed = atom::Feed::from_str(&xml)?;
    let items = feed.entries();
    for item in items {
        let mut is_in_categories: bool = categories.len() == 0;
        for wanted in categories {
            for cat in item.categories() {
                is_in_categories = is_in_categories || wanted == cat.term();
            }
        }
        if is_in_categories {
            let mut authors = String::new();
            for a in item.authors() {
                authors += a.name();
                authors += "\n";
            }
            authors.pop();
            let summary: Option<&str> = match item.summary() {
                Some(t) => Some(&t.value),
                None => None,
            };
            let link = if item.links().len() == 0 {
                None
            } else {
                Some(item.links()[0].href())
            };
            println!(
                "{}",
                conven::feed_item_str(
                    Some(&item.title().value),
                    summary,
                    Some(&authors),
                    link
                )
            );
        }
    }

    Ok(())
}

pub fn get_items_cache(
    cache: &mut UrlCache,
    categories: &Vec<String>,
    xml: &str,
    url: &str,
) -> Result<(), Box<dyn Error>> {
    let feed = atom::Feed::from_str(&xml)?;
    let items = feed.entries();
    for item in items {
        let id = item.id();
        let mut is_cached: bool = false;
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
                    is_in_categories = is_in_categories || wanted == cat.term();
                }
            }
            if is_in_categories {
                let mut authors = String::new();
                for a in item.authors() {
                    authors += a.name();
                    authors += "\n";
                }
                authors.pop();
                let summary: Option<&str> = match item.summary() {
                    Some(t) => Some(&t.value),
                    None => None,
                };
                let link = if item.links().len() == 0 {
                    None
                } else {
                    Some(item.links()[0].href())
                };
                println!(
                    "{}",
                    conven::feed_item_str(
                        Some(&item.title().value),
                        summary,
                        Some(&authors),
                        link
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
                });
            }
        }
    }
    Ok(())
}
