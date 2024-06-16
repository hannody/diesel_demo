use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let db_conn = &mut establish_connection();
    let pattern = format!("%{}%", target);
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(db_conn)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
