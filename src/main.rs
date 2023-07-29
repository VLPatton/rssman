use clap::{Parser, Subcommand};
use rss::Channel;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    categories: Vec<String>,

    #[arg(short, long, default_value_t = 0)]
    number: usize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fetch {
        #[arg(short, long, default_value_t = false)]
        no_use_cache: bool,
    },
    ClearCache,
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

    let cache_name = std::env::var_os("HOME")
        .unwrap_or(".".into())
        .to_str()
        .unwrap()
        .to_owned()
        + "/.cache/rssman/urls.json";

    let urls_txt = std::fs::read_to_string(urls)?;
    let urls = if args.number == 0 {
        urls_txt.lines().take(urls_txt.lines().count())
    } else {
        urls_txt.lines().take(args.number)
    };

    match args.command.unwrap_or(Commands::Fetch {
        no_use_cache: false,
    }) {
        Commands::Fetch { no_use_cache: nuc } => {
            if nuc {
                for url in urls {
                    let xml = reqwest::get(url).await?.bytes().await?;
                    let channel = Channel::read_from(&xml[..]);
                    match channel {
                        Ok(channel) => {
                            conven::rss::get_all_items(
                                &args.categories,
                                channel,
                            );
                        }
                        Err(rss::Error::InvalidStartTag) => {
                            let xml = String::from_utf8(xml.to_vec())?;
                            conven::atom::get_all_items(
                                &args.categories,
                                &xml,
                            )?;
                        }
                        Err(a) => Err(a)?,
                    }
                }
            } else {
                let mut cache: conven::UrlCache = serde_json::from_str(
                    &std::fs::read_to_string(&cache_name)?,
                )?;

                for url in urls {
                    let xml = reqwest::get(url).await?.bytes().await?;
                    let channel = Channel::read_from(&xml[..]);
                    match channel {
                        Ok(channel) => {
                            conven::rss::get_items_cache(
                                &mut cache,
                                &args.categories,
                                channel,
                                url,
                            );
                        }
                        Err(rss::Error::InvalidStartTag) => {
                            let xml = String::from_utf8(xml.to_vec())?;
                            conven::atom::get_items_cache(
                                &mut cache,
                                &args.categories,
                                &xml,
                                url,
                            )?;
                        }
                        Err(e) => Err(e)?,
                    }
                }

                std::fs::write(cache_name, serde_json::to_string(&cache)?)?;
            }
        }
        Commands::ClearCache => {
            std::fs::write(cache_name, r#"{"urls":[]}"#)?;
        }
    }

    Ok(())
}
