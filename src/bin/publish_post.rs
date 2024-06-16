use std::env::args;

use diesel::prelude::*;
use diesel_demo::*;

use self::models::Post;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("Publish_post requires a post ID <e.g. 1, 42, 1986.. etc>")
        .parse::<i32>()
        .expect("Invalid ID");

    let db_conn = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(db_conn)
        .unwrap();

    print!("Published post {}\n", post.title);
}
