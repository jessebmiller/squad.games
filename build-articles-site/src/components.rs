use maud::{html, Markup};

pub fn head(title: &str, stylesheets: Vec<&str>) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (title) }
            @for stylesheet in stylesheets {
                link rel="stylesheet" href=(format!("{}{}", "/", stylesheet)) {}
            }
         }
    }
}

pub fn header() -> Markup {
    html! {
        header {
            a href="/" {
                h1 { "Squad Games" }
            }
            nav {
                ul class="main-nav" {
                    li { a href="/" { "Home" } }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            h1 { "What is Squad Games?" }
            br;
            p { "Squad Games is a tabletop role playing game community and production studio created and organized by Jesse B. Miller. We create in the open, sharing ideas, prototypes, and play testing uncomfortably early. We invite collaboration and ideas from everyone who's got them."}
            p { "Join the " a href="https://discord.gg/Sc6vjTVbe5" { "Discord" } " and jump in." }
        }
    }
}
