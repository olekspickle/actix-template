use std::collections::HashMap;

use actix_web::{
    web::{self, Form, Html, Redirect},
    HttpResponse, Responder, Result,
};
use askama::Template;

use crate::db;

pub async fn home() -> Result<impl Responder> {
    let html = templates::Home {
        title: "Home".into(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub async fn hello(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let name = query.get("name").map_or("stranger", |l| l);
    let html = templates::Hello {
        name: name.to_string(),
        title: "Hello".into(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub async fn posts() -> Result<impl Responder> {
    // I found no way to simply redirect, actix way uses the default thing that
    // specifically designed to not refresh the page and it results in ugly double render
    //Ok(Redirect::to("/posts").see_other())
    let posts = db::get_all_posts().await.expect("getting all posts failed");
    log::trace!("fetched posts: {}", posts.len());

    let html = templates::Posts {
        title: "Posts".into(),
        posts,
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub async fn add_post(query: Form<db::NewPost>) -> Result<impl Responder> {
    db::add_post(query.into_inner())
        .await
        .expect("failed to add post");

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/posts"))
        .finish())
}

#[derive(serde::Deserialize)]
pub(crate) struct Id {
    id: u32,
}
pub async fn update_post(query: Form<db::NewPost>, path: web::Path<Id>) -> Result<impl Responder> {
    let id = path.into_inner().id;
    log::trace!("id:{id}, {query:?}");
    db::update_post(id, query.into_inner())
        .await
        .expect("failed to add post");

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/posts"))
        .finish())
}

pub async fn delete_post(query: web::Path<u32>) -> Result<impl Responder> {
    db::delete_post(query.into_inner())
        .await
        .expect("failed to add post");

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/posts"))
        .finish())
}

pub async fn not_found(path: web::Path<String>) -> Result<impl Responder> {
    log::info!("PATH {path}");
    let html = templates::NotFound {
        title: "404".into(),
        uri: path.to_string(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct Home {
        pub title: String,
    }

    #[derive(Template)]
    #[template(path = "hello.html")]
    pub struct Hello {
        pub title: String,
        pub name: String,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct Posts {
        pub title: String,
        pub posts: Vec<db::Post>,
    }

    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFound {
        pub title: String,
        pub uri: String,
    }
}
