// We can implement a trait on local type(as below)
// or local trait on external type (e.g. Summary on Vec<T>)
// ! but we can't implement external trait on external types (Display on Vec<T>)
use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more ...)") // default implementation
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SummaryDefault {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author() // no default implementation, but called
                                    // this means summarize_author must be implemented
        )
    }
}

// parameter 'item' must be of type that implements Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// above is a syntax sugar for a trait bound
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// we must use trait bound if we want to force several params be of the same type
pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

// we can also specify multiple trait bounds
pub fn notify4(item: &(impl Summary + Display)) {}
pub fn notify41<T: Summary + Display>(item: &T) {}

// trait bound with `where` clause
pub fn some_function<T,U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// Returning types that implement traits.
// -> especially useful for iterators and closures
// -> you can only use it if you return single type (won't work if your code
// returns Summary or Tweet)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// We can implement methods conditionally for types that implement specified traits
// by using a trait bound with 'impl' block with generic type params
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) { // this method works iff T impl 2 traits
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements 
// another trait. Those are called "blanket implementations".
// For ex: std library impl `ToString` trait on any type that impl Display trait
// impl<T: Display> ToString for T {} // -> a trait ToString implemented for T
// that impl Display


