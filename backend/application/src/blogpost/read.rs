use domain::models::BlogPost;
use infrastructure::establish_connection;
use diesel::prelude::*;

use crate::errors::ApplicationError;

pub fn list_blogpost(blogpost_id: i32) -> Result<BlogPost, ApplicationError> {
    use domain::schema::blog_posts;

    match blog_posts::table.find(blogpost_id).first::<BlogPost>(&mut establish_connection()) {
        Ok(blogpost) => Ok(blogpost),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = format!("Error finding blogpost with id {} - {}", blogpost_id, err);
                return Err(ApplicationError::NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_blogposts() -> Vec<BlogPost> {
    use domain::schema::blog_posts;

    match blog_posts::table.select(blog_posts::all_columns).load::<BlogPost>(&mut establish_connection()) {
        Ok(mut blog_posts) => {
            blog_posts.sort();
            blog_posts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
