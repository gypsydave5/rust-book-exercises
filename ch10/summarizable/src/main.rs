fn main() {
    println!("{}", summary_text(news_articles()));
    println!("{}", summary_text(tweets()));
    println!("{}", summary_text(feed()));
}

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("Read more now!")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summarizable for Box<Summarizable> {
    fn summary(&self) -> String {
        (**self).summary()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn summary_text<T: Summarizable>(articles: Vec<T>) -> String {
    let list : Vec<String> = articles.into_iter().map(|a| a.summary()).collect();
    list.join("\n")
}

fn feed() -> Vec<Box<Summarizable + 'static>> {
    vec![
        Box::new(NewsArticle {
            headline: String::from("Headline One"),
            author: String::from("Author One"),
            location: String::from("London"),
            content: String::from("stuff")
        }),
        Box::new(Tweet {
            username: String::from("DHH"),
            content: String::from("I hate whiteboards"),
            reply: true,
            retweet: true
        })
    ]
}

fn news_articles() -> Vec<NewsArticle> {
    vec![
        NewsArticle {
            headline: String::from("Headline One"),
            author: String::from("Author One"),
            location: String::from("London"),
            content: String::from("stuff")
        },
        NewsArticle {
            headline: String::from("Headline Two"),
            author: String::from("Author Two"),
            location: String::from("Liverpool"),
            content: String::from("even more stuff")
        }
    ]
}

fn tweets() -> Vec<Tweet> {
    vec![
        Tweet {
            username: String::from("gypsydave5"),
            content: String::from("Hello world"),
            reply: false,
            retweet: true,
        },
        Tweet {
            username: String::from("DHH"),
            content: String::from("I hate whiteboards"),
            reply: true,
            retweet: true
        }
    ]
}
