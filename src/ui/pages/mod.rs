use std::rc::Rc;

use crate::dao::JiraDAO;

mod epic_details;
mod home;
mod page;
mod page_helpers;
mod story_details;

pub use page::*;
pub use home::*;
pub use epic_details::*;
pub use story_details::*;

mod page_test_utils {
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
