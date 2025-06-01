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
    pub definition_header: String,
    pub definition: String,
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
