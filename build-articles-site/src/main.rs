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

    Ok(())
}
