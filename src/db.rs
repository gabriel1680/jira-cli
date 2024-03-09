use std::fs;

use anyhow::Result;

use crate::models::{DBState, Story, Epic, Status};

trait Database {
    fn retrieve(&self) -> Result<DBState>;
    fn persist(&self, state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub path: String
}

impl Database for JSONFileDatabase {
    fn retrieve(&self) -> Result<DBState> {
        let content = fs::read_to_string(&self.path)?;
        let state = serde_json::from_str(&content)?;
        Ok(state)
    }

    fn persist(&self, state: &DBState) -> Result<()> {
        fs::write(&self.path, &serde_json::to_vec(state)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn retrieve_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { path: "INVALID_PATH".to_owned() };
            assert_eq!(db.retrieve().is_err(), true);
        }

        #[test]
        fn retrieve_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.retrieve();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn retrieve_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.retrieve();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn persist_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.persist(&state);
            let read_result = db.retrieve().unwrap();

            assert_eq!(write_result.is_ok(), true);
            // TODO: fix this error by deriving the appropriate traits for DBState
            assert_eq!(read_result, state);
        }
    }
}

