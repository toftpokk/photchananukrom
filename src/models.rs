use diesel::{prelude::*, query_dsl::methods::FindDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::words)]
pub struct Word {
    id: i32,
    pub word: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Word))]
#[diesel(table_name = crate::schema::definitions)]
pub struct Definition {
    id: i32,
    word_id: i32,
    pub definition: String,
    pub definition_header: String,
}

#[derive(Deserialize, Debug)]
pub struct DefinitionBody(Vec<DefinitionBodyChild>);

impl DefinitionBody {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| match x {
                DefinitionBodyChild::String(s) => s.to_owned(),
                DefinitionBodyChild::Tag(t) => t.to_string(),
            })
            .collect()
    }
}

impl From<&String> for DefinitionBody {
    fn from(value: &String) -> Self {
        serde_json::from_str(value).unwrap()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DefinitionBodyTag {
    #[serde(rename = "type")]
    tag_type: String,
    attrs: Option<serde_json::Value>,
    children: Option<Vec<DefinitionBodyChild>>,
    to: Option<String>,
}

impl DefinitionBodyTag {
    fn to_string(&self) -> String {
        let children = if let Some(children) = &self.children {
            children
        } else {
            return "".to_string();
        };

        let child_str: String = children
            .iter()
            .map(|x| match x {
                DefinitionBodyChild::String(s) => s.to_string(),
                // TODO to_string is potentially unsafe
                DefinitionBodyChild::Tag(t) => t.to_string(),
            })
            .collect();
        match self.tag_type.as_str() {
            "br" => "<br>".to_string(),
            "sup" => format!(r#"<sup>{}</sup>"#, child_str),
            "sub" => format!(r#"<sub>{}</sub>"#, child_str),
            "i" => format!(r#"<i>{}</i>"#, child_str),
            "label-onclick" => format!(
                r#"<a href="?q={}">{}</a>"#,
                self.to
                    .clone()
                    .unwrap() // TODO potentially unsafe
                    .replace("lookupWord1('", "")
                    .replace("')", ""),
                child_str
            ),
            _ => child_str,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum DefinitionBodyChild {
    String(String),
    Tag(DefinitionBodyTag),
}

use diesel::result::Error as DieselError;
use std::fmt;

use crate::schema::{definitions, words};

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(DieselError),
    NotFound,
    ValidationError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RepositoryError::NotFound => write!(f, "Resource not found"),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl From<DieselError> for RepositoryError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => RepositoryError::NotFound,
            e => RepositoryError::DatabaseError(e),
        }
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

pub struct WordRepository;

impl WordRepository {
    pub fn find_by_word(conn: &mut SqliteConnection, word: String) -> RepositoryResult<Word> {
        let word = words::table.filter(words::word.eq(word)).first(conn)?;

        Ok(word)
    }
}

pub struct DefinitionRepository;

impl DefinitionRepository {
    pub fn find_by_word(
        conn: &mut SqliteConnection,
        word: &Word,
    ) -> RepositoryResult<Vec<Definition>> {
        let word = definitions::table
            .filter(definitions::word_id.eq(word.id))
            .load(conn)?;

        Ok(word)
    }
}

// TODO synonyms, antonyms, part of speech, pronunciation
/*
word
- string ephemeral
- pronunciation /əˈfem(ə)rəl/
- definitions
  - [0]
    part_of_speech noun
    - meanings
        - definition The occurrence and development of events by chance in a happy or
        beneficial way.
            - example A fortunate stroke of serendipity brought the two old friends together.
        - definition The occurrence and development of events by chance 2
            - example A fortunate stroke of serendipity brought
  - [1]
    part_of_speech adjective
    - meanings
        - definition Lasting for a very short time.
- synonyms
  - temporary
  - fleeting
  - transient
  - brief
- antonyms
  - permanent
  - lasting
  - enduring
- references
*/
