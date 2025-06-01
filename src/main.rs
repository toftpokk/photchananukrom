use std::{collections::HashMap, env};

use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    cookie::time::{OffsetDateTime, format_description::well_known::Rfc3339},
    get, post, web,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Meaning {
    definition: String,
    example: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Definition {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    meanings: Vec<Meaning>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WordData {
    word: String,
    pronunciation: String,
    definitions: Vec<Definition>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct SearchForm {
    word: String,
}

// Mock dictionary data - replace with database or API calls
fn get_mock_dictionary() -> HashMap<String, WordData> {
    let mut dict = HashMap::new();

    dict.insert("serendipity".to_string(), WordData {
        word: "serendipity".to_string(),
        pronunciation: "/ˌserənˈdipədē/".to_string(),
        definitions: vec![Definition {
            part_of_speech: "noun".to_string(),
            meanings: vec![Meaning {
                definition: "The occurrence and development of events by chance in a happy or beneficial way".to_string(),
                example: Some("A fortunate stroke of serendipity brought the two old friends together.".to_string()),
            }],
        }],
        synonyms: Some(vec!["chance".to_string(), "fortune".to_string(), "luck".to_string(), "fate".to_string()]),
        antonyms: Some(vec!["misfortune".to_string(), "design".to_string(), "intention".to_string()]),
    });

    dict.insert(
        "ephemeral".to_string(),
        WordData {
            word: "ephemeral".to_string(),
            pronunciation: "/əˈfem(ə)rəl/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "adjective".to_string(),
                meanings: vec![Meaning {
                    definition: "Lasting for a very short time".to_string(),
                    example: Some(
                        "The beauty of cherry blossoms is ephemeral, lasting only a few weeks."
                            .to_string(),
                    ),
                }],
            }],
            synonyms: Some(vec![
                "temporary".to_string(),
                "fleeting".to_string(),
                "transient".to_string(),
                "brief".to_string(),
            ]),
            antonyms: Some(vec![
                "permanent".to_string(),
                "lasting".to_string(),
                "enduring".to_string(),
                "eternal".to_string(),
            ]),
        },
    );

    dict.insert(
        "ubiquitous".to_string(),
        WordData {
            word: "ubiquitous".to_string(),
            pronunciation: "/yo͞oˈbikwədəs/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "adjective".to_string(),
                meanings: vec![Meaning {
                    definition: "Present, appearing, or found everywhere".to_string(),
                    example: Some(
                        "Smartphones have become ubiquitous in modern society.".to_string(),
                    ),
                }],
            }],
            synonyms: Some(vec![
                "omnipresent".to_string(),
                "pervasive".to_string(),
                "universal".to_string(),
                "widespread".to_string(),
            ]),
            antonyms: Some(vec![
                "rare".to_string(),
                "scarce".to_string(),
                "absent".to_string(),
                "limited".to_string(),
            ]),
        },
    );

    dict.insert(
        "perspicacious".to_string(),
        WordData {
            word: "perspicacious".to_string(),
            pronunciation: "/ˌpərspəˈkāSHəs/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "adjective".to_string(),
                meanings: vec![Meaning {
                    definition: "Having a ready insight into and understanding of things"
                        .to_string(),
                    example: Some(
                        "Her perspicacious analysis of the market trends impressed the board."
                            .to_string(),
                    ),
                }],
            }],
            synonyms: Some(vec![
                "perceptive".to_string(),
                "astute".to_string(),
                "shrewd".to_string(),
                "insightful".to_string(),
            ]),
            antonyms: Some(vec![
                "obtuse".to_string(),
                "dull".to_string(),
                "unperceptive".to_string(),
                "naive".to_string(),
            ]),
        },
    );

    dict.insert(
        "eloquent".to_string(),
        WordData {
            word: "eloquent".to_string(),
            pronunciation: "/ˈeləkwənt/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "adjective".to_string(),
                meanings: vec![
                    Meaning {
                        definition: "Fluent or persuasive in speaking or writing".to_string(),
                        example: Some(
                            "The politician delivered an eloquent speech that moved the audience."
                                .to_string(),
                        ),
                    },
                    Meaning {
                        definition: "Clearly expressing or indicating something".to_string(),
                        example: Some(
                            "Her silence was eloquent testimony to her disapproval.".to_string(),
                        ),
                    },
                ],
            }],
            synonyms: Some(vec![
                "articulate".to_string(),
                "fluent".to_string(),
                "expressive".to_string(),
                "persuasive".to_string(),
            ]),
            antonyms: Some(vec![
                "inarticulate".to_string(),
                "tongue-tied".to_string(),
                "hesitant".to_string(),
                "unclear".to_string(),
            ]),
        },
    );

    dict.insert(
        "aqaa".to_string(),
        WordData {
            word: "eloquent".to_string(),
            pronunciation: "/ˈeləkwənt/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "adjective".to_string(),
                meanings: vec![
                    Meaning {
                        definition: "Fluent or persuasive in speaking or writing".to_string(),
                        example: Some(
                            "The politician delivered an eloquent speech that moved the audience."
                                .to_string(),
                        ),
                    },
                    Meaning {
                        definition: "Clearly expressing or indicating something".to_string(),
                        example: Some(
                            "Her silence was eloquent testimony to her disapproval.".to_string(),
                        ),
                    },
                ],
            }],
            synonyms: Some(vec![
                "articulate".to_string(),
                "fluent".to_string(),
                "expressive".to_string(),
                "persuasive".to_string(),
            ]),
            antonyms: Some(vec![
                "inarticulate".to_string(),
                "tongue-tied".to_string(),
                "hesitant".to_string(),
                "unclear".to_string(),
            ]),
        },
    );

    dict
}

// Helper function to generate HTML for word tags
fn generate_word_tags(words: &[String], tag_type: &str) -> String {
    words
        .iter()
        .map(|word| {
            format!(
                r##"<span class="word-tag" hx-post="/search" hx-vals='{{"word": "{}"}}' hx-target="#results">{}</span>"##,
                word, word
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

// Generate HTML for word result
fn generate_word_result_html(word_data: &WordData) -> String {
    let definitions_html = word_data
        .definitions
        .iter()
        .map(|def| {
            let meanings_html = def
                .meanings
                .iter()
                .map(|meaning| {
                    let example_html = meaning
                        .example
                        .as_ref()
                        .map(|ex| format!(r#"<div class="example">"{}"</div>"#, ex))
                        .unwrap_or_default();

                    format!(
                        r#"<div class="definition">
                            <div class="definition-text">{}</div>
                            {}
                        </div>"#,
                        meaning.definition, example_html
                    )
                })
                .collect::<Vec<_>>()
                .join("");

            format!(
                r#"<div class="definition-group">
                    <div class="part-of-speech">{}</div>
                    {}
                </div>"#,
                def.part_of_speech, meanings_html
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let synonyms_html = word_data
        .synonyms
        .as_ref()
        .map(|synonyms| {
            format!(
                r#"<div class="synonyms">
                    <h4>Synonyms:</h4>
                    <div class="word-list">
                        {}
                    </div>
                </div>"#,
                generate_word_tags(synonyms, "synonym")
            )
        })
        .unwrap_or_default();

    let antonyms_html = word_data
        .antonyms
        .as_ref()
        .map(|antonyms| {
            format!(
                r#"<div class="antonyms">
                    <h4>Antonyms:</h4>
                    <div class="word-list">
                        {}
                    </div>
                </div>"#,
                generate_word_tags(antonyms, "antonym")
            )
        })
        .unwrap_or_default();

    format!(
        r#"<div class="word-result">
            <div class="word-header">
                <div class="word-title">{}</div>
                <div class="pronunciation">{}</div>
                <button class="audio-btn" onclick="playPronunciation('{}')">🔊 Play</button>
            </div>
            
            <div class="definitions">
                {}
            </div>
            
            {}
            {}
        </div>"#,
        word_data.word,
        word_data.pronunciation,
        word_data.word,
        definitions_html,
        synonyms_html,
        antonyms_html
    )
}

#[derive(Clone)]
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
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "dictionary-api"
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

#[post("/search")]
async fn search_word(form: web::Form<SearchForm>) -> impl Responder {
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

    let dictionary = get_mock_dictionary();

    match dictionary.get(&search_word) {
        Some(word_data) => {
            let html = generate_word_result_html(word_data);
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        None => {
            let error_html = format!(
                r#"<div class="error">
                    <h3>Word not found</h3>
                    <p>Sorry, we couldn't find "{}" in our dictionary. Try checking the spelling or searching for a different word.</p>
                </div>"#,
                search_word
            );
            HttpResponse::Ok()
                .content_type("text/html")
                .body(error_html)
        }
    }
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
