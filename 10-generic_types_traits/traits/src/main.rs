use traits::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // to use default implemenation we specify empty impl block like:
    // impl Summary for NewsArticle {} (in `lib.rs`)

    //
}
