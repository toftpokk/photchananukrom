use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    AppState, database, generate_word_result_html, models,
    models::{DefinitionRepository, WordRepository},
    templates,
};

use askama::Template;
use serde::Deserialize;

#[get("/health")]
pub async fn health(data: web::Data<AppState>) -> impl Responder {
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

#[derive(Debug, Deserialize)]
struct SearchForm {
    word: String,
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

    let error_html = templates::ErrorTemplate::new(&search_word)
        .render()
        .unwrap();

    let word = match WordRepository::find_by_word(&mut connection, search_word.clone()) {
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

    HttpResponse::Ok()
        .content_type("text/html")
        .append_header((
            "HX-Replace-Url",
            format!(
                "?q={}",
                percent_encoding::percent_encode(
                    search_word.as_bytes(),
                    percent_encoding::NON_ALPHANUMERIC,
                )
                .to_string()
            ),
        ))
        .body(html)
}
