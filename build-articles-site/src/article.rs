use std::path::PathBuf;
use gray_matter::Matter;
use gray_matter::engine::TOML;
use color_eyre::Result;
use serde::Deserialize;
use maud::{html, Markup, PreEscaped};

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: Option<String>,
    pub content: Option<String>,
    pub path: Option<PathBuf>,
    pub author: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
}

pub trait Summarizer {
    fn summary(&self) -> Markup;
}

impl Article {
    fn view_title(&self) -> Markup {
        html! {
            @if let Some(title) = &self.title {
                h1 class = "title" { (title) }
            } @else {
                h1 class = "title" { "Untitled" }
            }
        }
    }
}

impl Summarizer for Article {
    fn summary(&self) -> Markup {
        let summary = html! {
            div class="summary" {
                @if let Some(path) = self.url_path() {
                    a href=(path) {
                        (self.view_title())
                    }
                }
            }
        };
        summary
    }
}

pub trait UrlPather {
    fn url_path(&self) -> Option<String>;
}

impl UrlPather for Article {
    fn url_path(&self) -> Option<String> {
        self.path.clone().map(|mut p| {
            p.set_extension("html");
            p.to_string_lossy().to_string()
        })
    }
}

pub trait Viewable {
    fn view(&self) -> Markup;
}

impl Viewable for Article {
    fn view(&self) -> Markup {
        let view = html! {
            div class="article" {
                (self.view_title())
                @if let Some(content) = &self.content {
                    div class="content" {
                        (PreEscaped(markdown::to_html(content)))
                    }
                }
            }
        };

        view
    }
}

pub trait Articler {
    fn make_article(&self, strip_prefix: PathBuf) -> Result<Article>;
}

impl Articler for PathBuf {
    fn make_article(&self, strip_prefix: PathBuf) -> Result<Article> {
        println!("making article {} stripping prefix {}", self.display(), strip_prefix.display());
        let file_contents = std::fs::read_to_string(self).expect("Unable to read file");
        let matter: Matter<TOML> = Matter::new();
        let parsed = matter.parse(&file_contents);
        let parsed_article: Option<Article> = match parsed.data {
            Some(data) => match data.deserialize() {
                Ok(article) => Some(article),
                Err(e) => {
                    println!("Error deserializing article: {}", e);
                    None
                }
            },
            None => None,
        };
        let mut article = parsed_article.unwrap_or(Article {
            title: None,
            content: None,
            path: None,
            author: None,
            draft: None,
            tags: None,
            category: None,
        });
        article.content = Some(parsed.content);
        article.path = Some(
                self.clone().strip_prefix(strip_prefix).unwrap().to_path_buf()
        );
        Ok(article)
    }
}

pub trait ArticleIterator {
    fn get_articles(&self, strip_prefix: PathBuf) -> Result<Vec<Article>>;
}

impl ArticleIterator for PathBuf {
    fn get_articles(&self, strip_prefix: PathBuf) -> Result<Vec<Article>> {
        println!("Getting articles from: {} stripping prefix {}", self.display(), strip_prefix.display());
        let mut articles: Vec<Article> = Vec::new();
        for entry in std::fs::read_dir(self)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("md")) {
                println!("Found article: {}", path.to_str().unwrap());
                let article = path.make_article(strip_prefix.clone())?;
                articles.push(article);
            }
            if path.is_dir() {
                let mut sub_articles = path.get_articles(strip_prefix.clone())?;
                articles.append(&mut sub_articles);
            }
        }
        Ok(articles)
    }
}
