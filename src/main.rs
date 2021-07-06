use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let posts = Arc::new(vec![
        Post::new(String::from("a"), PostTemplate::new("hello")),
        Post::new(String::from("b"), PostTemplate::new("goodbye")),
    ]);
    let with_posts = warp::any().map(move || posts.clone());
    let routes =
        warp::path!(String)
            .and(with_posts)
            .and_then(|id, posts: Arc<Vec<Post>>| async move {
                match posts.iter().find(|post| post.slug == id) {
                    Some(v) => Ok(v.template),
                    None => Err(warp::reject::not_found()),
                }
            });
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

mod post {
    use askama::Template;
    #[derive(Template, Copy, Clone)]
    #[template(path = "post.html")]
    pub struct PostTemplate<'a> {
        content: &'a str,
    }

    impl<'a> PostTemplate<'a> {
        pub fn new(content: &'a str) -> Self {
            PostTemplate { content }
        }
    }
    pub struct Post {
        pub slug: String,
        pub template: PostTemplate<'static>,
    }

    impl Post {
        pub fn new(slug: String, template: PostTemplate<'static>) -> Self {
            Post { slug, template }
        }
    }
}

pub use self::post::{Post, PostTemplate};
