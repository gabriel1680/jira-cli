use std::fs;

use anyhow::{Ok, Result};

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn retrieve(&self) -> Result<DBState>;
    fn persist(&self, state: &DBState) -> Result<()>;
}

pub struct JiraDAO {
    database: Box<dyn Database>,
}

impl JiraDAO {
    pub fn new(path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.retrieve()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut state = self.database.retrieve()?;
        state.last_item_id += 1;
        state.epics.insert(state.last_item_id, epic);
        self.database.persist(&state)?;
        Ok(state.last_item_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut state = self.database.retrieve()?;
        let new_id = state.last_item_id + 1;
        let mut epic = state.epics.get_mut(&epic_id).unwrap();
        epic.stories.push(new_id);
        state.stories.insert(new_id, story);
        state.last_item_id = new_id;
        self.database.persist(&state)?;
        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let story_ids = state.epics.get(&epic_id).unwrap().stories.clone();
        state.stories.retain(|story_id, _| !story_ids.contains(story_id));
        state.epics.remove_entry(&epic_id).unwrap();
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let mut epic = state.epics.get_mut(&epic_id).unwrap();
        epic.stories.retain(|story| *story != story_id);
        state.stories.remove_entry(&story_id).unwrap();
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let mut epic = state.epics.get_mut(&epic_id).unwrap();
        epic.status = status;
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let mut story = state.stories.get_mut(&story_id).unwrap();
        story.status = status;
        self.database.persist(&state)?;
        Ok(())
    }
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

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn retrieve(&self) -> Result<DBState> {
            Ok(self.last_written_state.borrow().clone())
        }

        fn persist(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dao {
        use super::test_utils::MockDB;

        use super::*;

        fn make_sut() -> JiraDAO {
            JiraDAO {
                database: Box::new(MockDB::new()),
            }
        }

        fn empty_story() -> Story {
            Story::new("".to_owned(), "".to_owned())
        }

        fn empty_epic() -> Epic {
            Epic::new("".to_owned(), "".to_owned())
        }

        #[test]
        fn should_create_epic() {
            let db = make_sut();
            let epic = empty_epic();
            let result = db.create_epic(epic.clone());
            assert_eq!(result.is_ok(), true);

            let id = result.unwrap();
            let expected_id = 1;
            assert_eq!(id, expected_id);

            let db_state = db.read_db().unwrap();
            assert_eq!(db_state.last_item_id, expected_id);
            assert_eq!(db_state.epics.get(&id), Some(&epic));
        }

        // #[test]
        // fn create_story_should_error_if_invalid_epic_id() {
        //     let db = make_sut();
        //     let story = empty_story();
        //     let non_existent_epic_id = 999;
        //     let result = db.create_story(story, non_existent_epic_id);
        //     assert_eq!(result.is_err(), true);
        // }

        #[test]
        fn should_create_story() {
            let db = make_sut();
            let epic = empty_epic();
            let story = empty_story();
            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();
            let result = db.create_story(story.clone(), epic_id);
            assert_eq!(result.is_ok(), true);

            let id = result.unwrap();
            let db_state = db.read_db().unwrap();
            let expected_id = 2;
            assert_eq!(id, expected_id);
            assert_eq!(db_state.last_item_id, expected_id);
            assert_eq!(
                db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
                true
            );
            assert_eq!(db_state.stories.get(&id), Some(&story));
        }

        // #[test]
        // fn delete_epic_should_error_if_invalid_epic_id() {
        //     let db = make_sut();
        //     let non_existent_epic_id = 999;
        //     let result = db.delete_epic(non_existent_epic_id);
        //     assert_eq!(result.is_err(), true);
        // }

        #[test]
        fn should_delete_epic() {
            let db = make_sut();
            let epic = empty_epic();
            let story = empty_story();
            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();
            let result = db.create_story(story, epic_id);
            assert_eq!(result.is_ok(), true);

            let story_id = result.unwrap();
            let result = db.delete_epic(epic_id);
            assert_eq!(result.is_ok(), true);

            let db_state = db.read_db().unwrap();
            let expected_last_id = 2;
            assert_eq!(db_state.last_item_id, expected_last_id);
            assert_eq!(db_state.epics.get(&epic_id), None);
            assert_eq!(db_state.stories.get(&story_id), None);
        }

        // #[test]
        // fn delete_story_should_error_if_invalid_epic_id() {
        //     let db = make_sut();
        //     let epic = empty_epic();
        //     let story = empty_story();

        //     let result = db.create_epic(epic);
        //     assert_eq!(result.is_ok(), true);

        //     let epic_id = result.unwrap();
        //     let result = db.create_story(story, epic_id);
        //     assert_eq!(result.is_ok(), true);

        //     let story_id = result.unwrap();
        //     let non_existent_epic_id = 999;
        //     let result = db.delete_story(non_existent_epic_id, story_id);
        //     assert_eq!(result.is_err(), true);
        // }

        // #[test]
        // fn delete_story_should_error_if_story_not_found_in_epic() {
        //     let db = make_sut();
        //     let epic = empty_epic();
        //     let story = empty_story();

        //     let result = db.create_epic(epic);
        //     assert_eq!(result.is_ok(), true);

        //     let epic_id = result.unwrap();
        //     let result = db.create_story(story, epic_id);
        //     assert_eq!(result.is_ok(), true);

        //     let non_existent_story_id = 999;
        //     let result = db.delete_story(epic_id, non_existent_story_id);
        //     assert_eq!(result.is_err(), true);
        // }

        #[test]
        fn delete_story_should_work() {
            let db = make_sut();
            let epic = empty_epic();
            let story = empty_story();

            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();
            let result = db.create_story(story, epic_id);
            assert_eq!(result.is_ok(), true);

            let story_id = result.unwrap();
            let result = db.delete_story(epic_id, story_id);
            assert_eq!(result.is_ok(), true);

            let db_state = db.read_db().unwrap();
            let expected_last_id = 2;
            assert_eq!(db_state.last_item_id, expected_last_id);
            assert_eq!(
                db_state
                    .epics
                    .get(&epic_id)
                    .unwrap()
                    .stories
                    .contains(&story_id),
                false
            );
            assert_eq!(db_state.stories.get(&story_id), None);
        }

        // #[test]
        // fn update_epic_status_should_error_if_invalid_epic_id() {
        //     let db = make_sut();
        //     let non_existent_epic_id = 999;
        //     let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        //     assert_eq!(result.is_err(), true);
        // }

        #[test]
        fn update_epic_status_should_work() {
            let db = make_sut();
            let epic = empty_epic();

            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();
            let result = db.update_epic_status(epic_id, Status::Closed);
            assert_eq!(result.is_ok(), true);

            let db_state = db.read_db().unwrap();
            assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
        }

        // #[test]
        // fn update_story_status_should_error_if_invalid_story_id() {
        //     let db = make_sut();
        //     let non_existent_story_id = 999;
        //     let result = db.update_story_status(non_existent_story_id, Status::Closed);
        //     assert_eq!(result.is_err(), true);
        // }

        #[test]
        fn update_story_status_should_work() {
            let db = make_sut();
            let epic = empty_epic();
            let story = empty_story();
            let result = db.create_epic(epic);
            let epic_id = result.unwrap();
            let result = db.create_story(story, epic_id);
            let story_id = result.unwrap();
            let result = db.update_story_status(story_id, Status::Closed);
            assert_eq!(result.is_ok(), true);

            let db_state = db.read_db().unwrap();
            assert_eq!(
                db_state.stories.get(&story_id).unwrap().status,
                Status::Closed
            );
        }
    }

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
            run_against_file_with(r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#, test);
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
