mod post;
use post::Post;

pub fn demo() {
    println!("This is the state design pattern demo.");
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}

mod post2;

pub fn demo2() {
    println!("This is the state design pattern demo2.");
    let mut post = post2::Post::new();
    post.add_text("I ate a salad for lunch today.");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}
