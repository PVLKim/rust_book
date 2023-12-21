pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { 
            // .take method extract value from Some() and 
            // replaces it with None (self.state = None)
            // That's why Option is needed (Rust won't allow empty fields for structs)
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // internal method consumes the current state and return a new state
    // Box<Self> syntax means that method is valid only when called on a Box holding the type
    // It takes ownership of Box<Self>
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { // lifetime ~ Post, not State
        "" // default implementation for Draft and PendingReview states
    }
}
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {}) // this return new State
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // retain state 
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // this returns itself to keep in PendingReview state
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {}) // turn into Published state
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // do nothing
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // do nothing
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}