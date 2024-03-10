use std::fs;

use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn retrieve(&self) -> Result<DBState>;
    fn persist(&self, state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub path: String,
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

        fn run_against_file_with(content: &str, test: impl Fn(String) -> ()) {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            write!(tmpfile, "{}", content).unwrap();
            let path = tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str")
                .to_owned();
            test(path);
        }

        #[test]
        fn retrieve_should_fail_with_invalid_path() {
            let sut = JSONFileDatabase {
                path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(sut.retrieve().is_err(), true);
        }

        #[test]
        fn retrieve_should_fail_with_invalid_json() {
            let test = |path: String| {
                let sut = JSONFileDatabase { path };
                assert_eq!(sut.retrieve().is_err(), true);
            };
            run_against_file_with(r#"{ "last_item_id": 0 epics: {} stories {} }"#, test);
        }

        #[test]
        fn retrieve_should_parse_json_file() {
            let test = |path: String| {
                let sut = JSONFileDatabase { path };
                assert_eq!(sut.retrieve().is_ok(), true);
            };
            run_against_file_with(
                r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#,
                test,
            );
        }

        #[test]
        fn persist_should_work() {
            let test = |path: String| {
                let db = JSONFileDatabase { path };

                let story = Story {
                    name: "epic 1".to_owned(),
                    description: "epic 1".to_owned(),
                    status: Status::Open,
                };
                let epic = Epic {
                    name: "epic 1".to_owned(),
                    description: "epic 1".to_owned(),
                    status: Status::Open,
                    stories: vec![2],
                };

                let mut stories = HashMap::new();
                stories.insert(2, story);

                let mut epics = HashMap::new();
                epics.insert(1, epic);

                let state = DBState {
                    last_item_id: 2,
                    epics,
                    stories,
                };

                assert_eq!(db.persist(&state).is_ok(), true);
                assert_eq!(db.retrieve().unwrap(), state);
            };
            let json = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            run_against_file_with(json, test);
        }
    }
}
