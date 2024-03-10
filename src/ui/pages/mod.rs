use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

use crate::dao::JiraDAO;
use crate::models::Action;

mod epic_details;
mod home_page;
mod page_helpers;
mod story_details;

use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
}

pub mod page_test_utils {
    use super::*;
    use crate::dao::test_utils::MockDB;
    use crate::models::{Epic, Story};

    pub fn make_dao() -> Rc<JiraDAO> {
        let database = Box::new(MockDB::new());
        Rc::new(JiraDAO::new(database))
    }

    pub fn create_epic_and_story(dao: &JiraDAO) -> (u32, u32) {
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let story_id = dao
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .unwrap();
        (epic_id, story_id)
    }
}
