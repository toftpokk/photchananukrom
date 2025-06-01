use std::env;

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    cookie::time::{OffsetDateTime, format_description::well_known::Rfc3339},
    get, post, web,
};
use askama::Template;
use models::{DefinitionRepository, WordRepository};

mod database;
mod models;
mod schema;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchForm {
    word: String,
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
        <span class="word-tag" hx-post="/search" hx-vals='{"word": "{{ word }}"}' hx-target="#results">{{ word }}</span>
    "##
)]
struct WordTagTemplate<'a> {
    word: &'a String,
}

// Helper function to generate HTML for word tags
fn generate_word_tags(words: &[String], tag_type: &str) -> String {
    words
        .iter()
        .map(|word: &String| WordTagTemplate { word: word }.render().unwrap())
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
    <div class="word-result">
        <div class="word-header">
            <div class="word-title">{{ title }}</div>
        </div>
        <div class="definitions">
        {% for definition in definitions %}
            <div class="definition-group">
                <div class="definition-text">{{ definition.definition_header }}</div>
            </div>
            {{ definition.definition }}
        {% endfor %}
        </div>
    </div>
    "#
)]
struct WordResultTemplate<'a> {
    title: &'a String,
    definitions: Vec<models::Definition>,
}

// Generate HTML for word result
fn generate_word_result_html(word: &models::Word, definitions: Vec<models::Definition>) -> String {
    return WordResultTemplate {
        title: &word.word,
        definitions: definitions,
    }
    .render()
    .unwrap();
}

struct AppState {
    version: String,
    build_date: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let html = include_str!("../templates/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[get("/health")]
async fn health(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "dictionary-api",
        "version": data.version,
        "build_date": data.build_date,
    }))
}

#[get("/quick-links")]
async fn quick_links() -> impl Responder {
    let quick_links = r##"
    <div class="quick-link" hx-post="/search" hx-vals='{"word": "eloquent"}' hx-target="#results">eloquent
    </div>
    "##;
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(quick_links);
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
    <div class="error">
        <h3>Word not found</h3>
        <p>Sorry, we couldn't find "{{ search_word }}" in our dictionary. Try checking the spelling or searching for a different word.</p>
    </div>
    "#
)]
struct ErrorTemplate<'a> {
    search_word: &'a String,
}

#[post("/search")]
async fn search_word(form: web::Form<SearchForm>) -> impl Responder {
    let mut connection = database::establish_connection();

    // TODO dependency injection
    let search_word = form.word.trim().to_lowercase();

    // Handle empty search
    if search_word.is_empty() {
        let welcome_html = r#"
            <div class="welcome-message">
                <h3>Photchananukrom</h3>
                <p>น. เว็บไซต์รวบรวมคำและความหมายภาษาไทย ดู พจนานุกรม</p>
            </div>
        "#;
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(welcome_html);
    }

    let error_html = ErrorTemplate {
        search_word: &search_word,
    }
    .render()
    .unwrap();

    let word = match WordRepository::find_by_word(&mut connection, search_word) {
        Ok(word) => word,
        Err(err) => match err {
            models::RepositoryError::NotFound => {
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(error_html);
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
                return HttpResponse::Ok()
                    .content_type("text/html")
                    .body(error_html);
            }
            err => {
                panic!("{}", err)
            }
        },
    };

    let html = generate_word_result_html(&word, definitions);

    HttpResponse::Ok().content_type("text/html").body(html)
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
            .service(health)
            .service(index)
            .service(search_word)
            .service(quick_links)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
