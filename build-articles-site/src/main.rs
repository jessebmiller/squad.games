mod article;
mod components;

use std::env;
use color_eyre::eyre::Result;
use maud::html;
use std::path::PathBuf;
use crate::article::{Summarizer, ArticleIterator, Viewable, UrlPather};
use crate::components::{head, header, footer};


fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a path to the articles content root folder\n\nUsage: build ../articles");
    }

    let root = PathBuf::from(&args[1]);
    let articles = root.get_articles(root.clone())?;

    // TODO consider bulma
    let stylesheets = vec!["reset.css", "site.css", "article.css"];

    let index = html! {
        html {
            (head("Squad Games | Articles", stylesheets.clone()))
            body {
                (header())
                div class="content" {
                    ul {
                        @for article in articles.iter().filter(|a| a.draft == Some(false)) {
                            li { (article.summary()) }
                        }
                    }
                }
                (footer())
            }
        }
    };

    let build_folder = PathBuf::from(".").join("build");
    if build_folder.exists() {
        std::fs::remove_dir_all(&build_folder)?;
    }
    std::fs::create_dir_all(build_folder.clone())?;
    std::fs::write(&build_folder.join("index.html"), index.into_string())?;

    stylesheets.iter().for_each(|s| {
        match std::fs::copy(root.join(s), build_folder.join(s)) {
            Ok(_) => (),
            Err(e) => println!("Failed to copy {:?} to build folder {:?}: {}", root.join(s), build_folder.join(s), e)
        }
    });

    for article in articles.iter().filter(|a| a.draft == Some(false)) {
        let title = article.title.clone().ok_or("Untitled").unwrap();
        let article_html = html! {
            html {
                (head(&title, stylesheets.clone()))
                body {
                    (header())
                    (article.view())
                    (footer())
                }
            }
        };

        match article.url_path() {
            Some(path) => {
                std::fs::write(&build_folder.join(path), article_html.into_string())?;
            }
            None => {
                println!("Article ({:?}) has no path", title);
            }
        }
    }

    let mut channel = rss::ChannelBuilder::default()
        .title("Squad Games".to_string())
        .link("https://squad.games".to_string())
        .description("Squad Games is a tabletop role playing game community and production studio created and organized by Jesse B. Miller. We create in the open, sharing ideas, prototypes, and play testing uncomfortably early. We invite collaboration and ideas from everyone who's got them.".to_string())
        .build();

    let mut rss_items: Vec<rss::Item> = Vec::new();
    for article in articles.iter().filter(|a| a.draft == Some(false)) {
        let title = article.title.clone().ok_or("Untitled").unwrap();
        let url = format!("https://squad.games/{}", article.url_path().unwrap());
        let item = rss::ItemBuilder::default()
            .title(Some(title))
            .link(Some(url))
            .description(Some(article.summary().into_string()))
            .content(Some(article.view().into_string()))
            .build();
        rss_items.push(item);
    }

    channel.set_items(rss_items);
    channel.write_to(
        std::fs::File::create(build_folder.join("feed.xml"))?
    )?;

    Ok(())
}
