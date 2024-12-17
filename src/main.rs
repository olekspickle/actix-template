//! ![actix-template](https://github.com/user-attachments/assets/91d5c75d-e809-4b22-98c3-d9afff07164d)
//!
//! ## Overview
//! Template to have something to get-go in some situations
//!
//! This template provides:
//! - [x] Actix server(with middleware)
//! - [x] Templates
//! - [x] Containerization
//! - [x] simple Sqlite integration setup with connection pool(deadpool)
//!
//! ## Afterthoughts and issues
//! Even if actix has some performance wins,
//! I generally found it less ergonomic and convenient than axum
//! It was still fun to check it's current state and I think that maybe it will
//! do better user experience oriented solutions. My immediate painpoints were:
//! - I could not figure out simple 404 default route handling, like in axum it's simply .not_found_service
//! - Reroute behaves strangely and rerenders the templates instead of just rerouting to another
//!     handler, but it might be how the 308 status code behavior specifically works,
//!     it does not really matter - in axum it just works
//! - For some reason PATCH handler simply 404 the form patch request from `/update-post/1` endpoint, I
//!     was too lazy to figure it out
//! - The last might be called a nitpick, but log over tracing? Really? At this point I am just so
//!     used for tracing being an industry standart that for me it would be a huge pain, at least until
//!     I study log docs as much as I have tracing ecosystem.
//!

use actix_files::Files;
use actix_web::{
    middleware,
    web::{get, post, resource, to, JsonConfig},
    App, HttpServer,
};

mod db;
mod handlers;
mod mw;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    db::init().await?;

    let addr = ("0.0.0.0", 7777);
    HttpServer::new(move || {
        // Set a payload limit
        let json_cfg = JsonConfig::default().limit(4096);
        App::new()
            .app_data(json_cfg)
            .wrap(middleware::Logger::default())
            .wrap(middleware::from_fn(mw::not_found))
            .service(
                Files::new("/static", "./static")
                    .show_files_listing()
                    .index_file("custom.css"),
            )
            .service(resource("/").route(get().to(handlers::home)))
            .service(resource("/posts").route(get().to(handlers::posts)))
            .service(resource("/hello").route(get().to(handlers::hello)))
            .service(resource("/add-post").route(post().to(handlers::add_post)))
            .service(resource("/update-post/{id}").route(post().to(handlers::update_post)))
            .service(resource("/delete-post/{id}").route(post().to(handlers::delete_post)))
            .default_service(to(handlers::not_found))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

/// use openssl to generate ssl certs
/// openssl req -newkey rsa:2048 -new -nodes -keyout key.pem -out csr.pem
///
/// or for dev purposes
///
/// openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem -addext "subjectAltName = DNS:mydnsname.com"
fn _load_rustls_config() -> rustls::ServerConfig {
    use std::{fs::File, io::BufReader};

    use rustls::{pki_types::PrivateKeyDer, ServerConfig};
    use rustls_pemfile::{certs, pkcs8_private_keys};

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
