use summary_trait::{NewsArticle, Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
    notify_t(&tweet);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_t<T: Summary>(item: &T) {
    println!("Breaking news with T! {}", item.summarize());
}