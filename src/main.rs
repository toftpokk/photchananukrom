use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    cookie::time::{OffsetDateTime, format_description::well_known::Rfc3339},
    get, web,
};

mod api;
mod database;
mod models;
mod schema;
mod templates;

use askama::Template;
use models::{DefinitionRepository, WordRepository};
use serde::Deserialize;

// Helper function to generate HTML for word tags
fn generate_word_tags(words: &[String], tag_type: &str) -> String {
    words
        .iter()
        .map(|word: &String| templates::WordTag::new(word).render().unwrap())
        .collect::<Vec<_>>()
        .join("")
}

// Generate HTML for word result
fn generate_word_result_html(word: &models::Word, definitions: Vec<models::Definition>) -> String {
    let definition_list = definitions
        .iter()
        .map(|x| {
            templates::Definition::new(
                x.definition_header.to_string(),
                models::DefinitionBody::from(&x.definition).to_string(),
            )
        })
        .collect();
    return templates::WordResult::new(&word.word, definition_list)
        .render()
        .unwrap();
}

fn generate_query_result(search_word: &String) -> Result<String, ()> {
    let mut connection = database::establish_connection();

    // Handle empty search
    if search_word.is_empty() {
        let welcome_html = r#"
            <div class="welcome-message">
                <h3>Photchananukrom</h3>
                <p>น. เว็บไซต์รวบรวมคำและความหมายภาษาไทย ดู พจนานุกรม</p>
            </div>
        "#;
        return Ok(welcome_html.to_string());
    }

    let word = match WordRepository::find_by_word(&mut connection, search_word.clone()) {
        Ok(word) => word,
        Err(err) => match err {
            models::RepositoryError::NotFound => {
                return Err(());
            }
            err => {
                panic!("{}", err)
            }
        },
    };

    let definitions = match DefinitionRepository::find_by_word(&mut connection, &word) {
        Ok(defs) => defs,
        Err(err) => match err {
            models::RepositoryError::NotFound => {
                return Err(());
            }
            err => {
                panic!("{}", err)
            }
        },
    };

    Ok(generate_word_result_html(&word, definitions))
}

struct AppState {
    version: String,
    build_date: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexRequest {
    q: Option<String>,
}

#[get("/")]
async fn index(query: web::Query<IndexRequest>) -> impl Responder {
    let search_word = query.q.clone().unwrap_or("".to_string());
    let word = search_word.trim().to_lowercase();

    let error_html = templates::Error::new(&word).render().unwrap();
    let res = match generate_query_result(&word) {
        Ok(html) => html,
        Err(_) => error_html,
    };

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(templates::Index::new(res).render().unwrap());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let build_date = match env::var("SOURCE_DATE_EPOCH") {
        Ok(date) => OffsetDateTime::from_unix_timestamp(date.parse::<i64>().unwrap())
            .unwrap()
            .format(&Rfc3339)
            .unwrap(),
        Err(_) => "".to_string(),
    };

    let version = match env::var("VERSION") {
        Ok(version) => version,
        Err(_) => "".to_string(),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                version: version.clone(),
                build_date: build_date.clone(),
            }))
            .service(api::health)
            .service(api::search_word)
            .service(api::quick_links)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
