use askama::Template;

#[derive(Debug)]
pub struct Definition {
    definition_header: String,
    definition_body: String,
}

impl Definition {
    pub fn new(definition_header: String, definition_body: String) -> Self {
        Self {
            definition_header,
            definition_body,
        }
    }
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
            {{ definition.definition_header }}
            <div class="definition-group">
                <div class="definition-text">{{ definition.definition_body | safe }}</div>
            </div>
        {% endfor %}
        </div>
    </div>
    "#
)]
pub struct WordResult<'a> {
    title: &'a String,
    definitions: Vec<Definition>,
}

impl<'a> WordResult<'a> {
    pub fn new(title: &'a String, definitions: Vec<Definition>) -> Self {
        Self { title, definitions }
    }
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
        <span class="word-tag" hx-post="/search" hx-vals='{"word": "{{ word }}"}' hx-target="#results">{{ word }}</span>
    "##
)]
pub struct WordTag<'a> {
    word: &'a String,
}

impl<'a> WordTag<'a> {
    pub fn new(word: &'a String) -> Self {
        Self { word }
    }
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
pub struct Error<'a> {
    search_word: &'a String,
}

impl<'a> Error<'a> {
    pub fn new(search_word: &'a String) -> Self {
        Self { search_word }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    query_result: String,
}
impl Index {
    pub fn new(query_result: String) -> Self {
        Self { query_result }
    }
}

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
    {% for link in links %}
        <div class="quick-link" hx-post="/search" hx-vals='{"word": "{{ link.0 }}"}' hx-target="#results">
            {{ link.1 }}
        </div>
    {% endfor %}
    "##
)]
pub struct QuickLinks<'a> {
    links: Vec<(&'a str, &'a str)>,
}

impl<'a> QuickLinks<'a> {
    pub fn new(links: Vec<(&'a str, &'a str)>) -> Self {
        Self { links }
    }
}
