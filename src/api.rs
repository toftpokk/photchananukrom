use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    AppState, database, generate_query_result, generate_word_result_html,
    models::{self, DefinitionRepository, WordRepository},
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
    let word = form.word.trim().to_lowercase();

    let error_html = templates::Error::new(&word).render().unwrap();
    let res = match generate_query_result(&word) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .append_header((
                "HX-Replace-Url",
                format!(
                    "?q={}",
                    percent_encoding::percent_encode(
                        word.as_bytes(),
                        percent_encoding::NON_ALPHANUMERIC,
                    )
                    .to_string()
                ),
            ))
            .body(html),
        Err(_) => {
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(error_html);
        }
    };
    res
}
