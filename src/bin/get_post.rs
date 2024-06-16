use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

pub fn main() {
    use self::schema::posts::dsl::{posts, published};

    let post_id = args()
        .nth(1)
        .expect("Enter the Post ID to fetch <e.g. 1, 42, 1986.. etc>")
        .parse::<i32>()
        .expect("Invalid ID");

    let db_conn = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(db_conn)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}
