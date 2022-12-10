mod api;
mod model;
mod repository;

use api::task::{get_task, TaskIdentifier};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::logger;
use model::task::Task;
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    sync::Client,
    sync::Collection,
    Database,
};

use repository::ddb::TaskService;
use std::env;

pub struct DDBContainer {
    task: TaskService,
}

impl DDBContainer {
    pub fn new(task: TaskService) -> Self {
        DDBContainer { task }
    }
}

pub struct AppState {
    ddb_container: DDBContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let client_result: Result<Client, Error> = connect_db().await;

    let client = match client_result {
        Ok(value) => value,
        _ => panic!("Couldn't get the client for the database"),
    };

    let db = client.database("Birdnest");

    let task_collection: Collection<Task> = db.collection("Task");

    HttpServer::new(move || {
        let ddb_container = DDBContainer::new(TaskService::new(task_collection.clone()));
        let logger = Logger::default();
        App::new()
            .app_data(AppState { ddb_container })
            .wrap(logger)
            .service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}

async fn connect_db() -> Result<Client, Error> {
    let connection_string = env::var("MONGODB_CONNECTION_STRING")
        .expect("$MONGODB_CONNECTION_STRING has not been set!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    // let options =
    //     ClientOptions::parse_with_resolver_config(&connection_string, ResolverConfig::cloudflare());
    let options = mongodb::options::ClientOptions::parse(&connection_string).unwrap();
    Client::with_options(options)
}
