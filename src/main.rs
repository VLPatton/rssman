use atom_syndication as atom;
use clap::Parser;
use rss::Channel;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    categories: Vec<String>,

    #[arg(short, long, default_value_t = 0)]
    number: usize,
}

mod conven;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let urls = std::env::var_os("HOME")
        .unwrap_or(".".into())
        .to_str()
        .unwrap()
        .to_owned()
        + "/.config/rssman/urls.txt";

    let urls_txt = std::fs::read_to_string(urls)?;
    let urls = if args.number == 0 {
        urls_txt.lines().take(urls_txt.lines().count())
    } else {
        urls_txt.lines().take(args.number)
    };

    for url in urls {
        let xml = reqwest::get(url).await?.bytes().await?;
        let channel = Channel::read_from(&xml[..]);
        match channel {
            Ok(channel) => {
                let items = channel.into_items();
                for item in items {
                    let mut is_in_categories: bool = args.categories.len() == 0;
                    for wanted in &args.categories {
                        for cat in item.categories() {
                            is_in_categories =
                                is_in_categories || wanted == cat.name();
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
            Err(rss::Error::InvalidStartTag) => {
                let xml = String::from_utf8(xml.to_vec())?;
                let feed = atom::Feed::from_str(&xml)?;
                let items = feed.entries();
                for item in items {
                    let mut is_in_categories: bool = args.categories.len() == 0;
                    for wanted in &args.categories {
                        for cat in item.categories() {
                            is_in_categories =
                                is_in_categories || wanted == cat.term();
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
            }
            Err(a) => Err(a)?,
        }
    }

    Ok(())
}
