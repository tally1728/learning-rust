// State Pattern
pub mod state_pattern {
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
            self.content
                .push_str(self.state.as_ref().unwrap().add_text(text));
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject());
            }
        }
    }

    // Trait Object
    trait State {
        fn add_text<'a>(&self, _text: &'a str) -> &'a str {
            ""
        }
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    // Initial State
    struct Draft {}

    impl State for Draft {
        fn add_text<'a>(&self, text: &'a str) -> &'a str {
            text
        }

        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    // 2nd State
    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingAnotherReview {})
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    // 3rd State
    struct PendingAnotherReview {}

    impl State for PendingAnotherReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    // Final State
    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Post;

        #[test]
        fn test1() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn test2_rejct() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.reject();
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn test3_add_text() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.add_text("NOISE");
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.add_text("NOISE");
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}

// 2. Encoding State & Behavior as Types
pub mod encoding_state {
    // Constructor & Final State
    pub struct Post {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    // Initial State
    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    // 2nd State
    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> PendingAnotherReviewPost {
            PendingAnotherReviewPost {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    // 3rd State
    pub struct PendingAnotherReviewPost {
        content: String,
    }

    impl PendingAnotherReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Post;

        #[test]
        fn test() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            // Request & Reject
            let post = post.request_review();
            let post = post.reject();

            // Request & 1 Approval & Reject
            let post = post.request_review();
            let post = post.approve();
            let post = post.reject();

            // Request & 2 Approvals
            let post = post.request_review();
            let post = post.approve();
            let post = post.approve();

            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}
