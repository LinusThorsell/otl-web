use shared::response_models::{Response, ResponseBody};
use application::blogpost::read;
use domain::models::BlogPost;
use rocket::get;
use rocket::response::status::NotFound;

#[get("/blogposts")]
pub fn list_blogposts_handler() -> String {
    let blogposts: Vec<BlogPost> = read::list_blogposts();
    let response = Response { body: ResponseBody::BlogPosts(blogposts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/blogpost/<blogpost_id>")]
pub fn list_blogpost_handler(blogpost_id: i32) -> Result<String, NotFound<String>> {
    let blogpost = read::list_blogpost(blogpost_id)?;
    let response = Response { body: ResponseBody::BlogPost(blogpost) };

    Ok(serde_json::to_string(&response).unwrap())
}
