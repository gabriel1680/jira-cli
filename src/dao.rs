use anyhow::{anyhow, Ok, Result};

use crate::models::{DBState, Epic, Status, Story};

pub trait Database {
    fn retrieve(&self) -> Result<DBState>;
    fn persist(&self, state: &DBState) -> Result<()>;
}

pub struct JiraDAO {
    database: Box<dyn Database>,
}

impl JiraDAO {
    pub fn new(database: Box<dyn Database>) -> JiraDAO {
        JiraDAO { database }
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
        state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Couldn't find epic in database"))?
            .stories
            .push(new_id);
        state.stories.insert(new_id, story);
        state.last_item_id = new_id;
        self.database.persist(&state)?;
        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut state = self.database.retrieve()?;
        for story_id in &state
            .epics
            .get(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?
            .stories
        {
            state.stories.remove(story_id);
        }
        state.epics.remove(&epic_id);
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let epic = state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?;
        let story_index = epic
            .stories
            .iter()
            .position(|id| id == &story_id)
            .ok_or_else(|| anyhow!("story id not found in epic stories vector"))?;
        epic.stories.remove(story_index);
        state.stories.remove(&story_id);
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let mut epic = state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("epic id not found"))?;
        epic.status = status;
        self.database.persist(&state)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut state = self.database.retrieve()?;
        let mut story = state
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("story not found"))?;
        story.status = status;
        self.database.persist(&state)?;
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

    use super::test_utils::MockDB;

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

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = make_sut();
        let story = empty_story();
        let non_existent_epic_id = 999;
        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

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

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = make_sut();
        let non_existent_epic_id = 999;
        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_delete_epic() {
        let db = make_sut();
        let epic = empty_epic();
        let story = empty_story();
        let epic_id = db.create_epic(epic).unwrap();
        let story_id = db.create_story(story, epic_id).unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        let expected_last_id = 2;
        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = make_sut();
        let epic = empty_epic();
        let story = empty_story();
        let epic_id = db.create_epic(epic).unwrap();
        let story_id = db.create_story(story, epic_id).unwrap();
        let non_existent_epic_id = 999;

        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = make_sut();
        let epic = empty_epic();
        let story = empty_story();
        let epic_id = db.create_epic(epic).unwrap();
        db.create_story(story, epic_id).unwrap();
        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = make_sut();
        let epic = empty_epic();
        let story = empty_story();

        let epic_id = db.create_epic(epic).unwrap();
        let story_id = db.create_story(story, epic_id).unwrap();
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

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = make_sut();
        let non_existent_epic_id = 999;
        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = make_sut();
        let epic = empty_epic();

        let epic_id = db.create_epic(epic).unwrap();
        let result = db.update_epic_status(epic_id, Status::Closed);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = make_sut();
        let non_existent_story_id = 999;
        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

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
