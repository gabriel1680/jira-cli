use anyhow::{anyhow, Context, Ok, Result};
use std::rc::Rc;

use crate::{
    dao::JiraDAO,
    ui::{Action, EpicDetail, HomePage, Page, Prompts, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    dao: Rc<JiraDAO>,
}

impl Navigator {
    pub fn new(dao: Rc<JiraDAO>) -> Self {
        Self {
            pages: vec![Box::new(HomePage {
                dao: Rc::clone(&dao),
            })],
            prompts: Prompts::new(),
            dao,
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                self.pages.push(Box::new(EpicDetail {
                    dao: Rc::clone(&self.dao),
                    epic_id,
                }));
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                self.pages.push(Box::new(StoryDetail {
                    dao: Rc::clone(&self.dao),
                    story_id,
                    epic_id,
                }));
            }
            Action::NavigateToPreviousPage => {
                if !self.pages.is_empty() {
                    self.pages.pop();
                }
            }
            Action::CreateEpic => {
                self.dao
                    .create_epic((self.prompts.create_epic)())
                    .with_context(|| anyhow!("failed to create a new epic"))?;
            }
            Action::UpdateEpicStatus { epic_id } => {
                if let Some(status) = (self.prompts.update_status)() {
                    self.dao
                        .update_epic_status(epic_id, status)
                        .with_context(|| anyhow!("failed to update epic"))?;
                }
            }
            Action::DeleteEpic { epic_id } => {
                if (self.prompts.delete_epic)() {
                    self.dao
                        .delete_epic(epic_id)
                        .with_context(|| anyhow!("failed to delete epic!"))?;
                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::CreateStory { epic_id } => {
                self.dao
                    .create_story((self.prompts.create_story)(), epic_id)
                    .with_context(|| anyhow!("failed to create a new story"))?;
            }
            Action::UpdateStoryStatus { story_id } => {
                if let Some(status) = (self.prompts.update_status)() {
                    self.dao
                        .update_story_status(story_id, status)
                        .with_context(|| anyhow!("failed to update story"))?;
                }
            }
            Action::DeleteStory { epic_id, story_id } => {
                if (self.prompts.delete_story)() {
                    self.dao
                        .delete_story(epic_id, story_id)
                        .with_context(|| anyhow!("failed to delete story"))?;
                    if !self.pages.is_empty() {
                        self.pages.pop();
                    }
                }
            }
            Action::Exit => {
                self.pages.clear();
            }
        }

        Ok(())
    }

    // Private functions used for testing

    fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dao::test_utils::MockDB,
        models::{Epic, Status, Story},
        ui::{EpicDetail, HomePage, StoryDetail},
    };

    fn make_dao() -> Rc<JiraDAO> {
        Rc::new(JiraDAO::new(Box::new(MockDB::new())))
    }

    fn make_sut() -> Navigator {
        Navigator::new(make_dao())
    }

    #[test]
    fn should_start_on_home_page() {
        let sut = make_sut();
        let current_page = sut.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();
        assert_eq!(sut.get_page_count(), 1);
        assert_eq!(home_page.is_some(), true);
    }

    #[test]
    fn handle_action_should_navigate_pages() {
        let mut sut = make_sut();

        sut.handle_action(Action::NavigateToEpicDetail { epic_id: 1 })
            .unwrap();
        assert_eq!(sut.get_page_count(), 2);

        let current_page = sut.get_current_page().unwrap();
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        sut.handle_action(Action::NavigateToStoryDetail {
            epic_id: 1,
            story_id: 2,
        })
        .unwrap();
        assert_eq!(sut.get_page_count(), 3);

        let current_page = sut.get_current_page().unwrap();
        let story_detail_page = current_page.as_any().downcast_ref::<StoryDetail>();
        assert_eq!(story_detail_page.is_some(), true);

        sut.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(sut.get_page_count(), 2);

        let current_page = sut.get_current_page().unwrap();
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        sut.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(sut.get_page_count(), 1);

        let current_page = sut.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();
        assert_eq!(home_page.is_some(), true);

        sut.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(sut.get_page_count(), 0);

        sut.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(sut.get_page_count(), 0);
    }

    #[test]
    fn handle_action_should_clear_pages_on_exit() {
        let mut sut = make_sut();
        sut.handle_action(Action::Exit).unwrap();
        assert_eq!(sut.get_page_count(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_epic() {
        let dao = make_dao();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.create_epic = Box::new(|| Epic::new("name".to_owned(), "description".to_owned()));
        sut.set_prompts(prompts);

        sut.handle_action(Action::CreateEpic).unwrap();

        let db_state = dao.read_db().unwrap();
        assert_eq!(db_state.epics.len(), 1);
        let epic = db_state.epics.into_iter().next().unwrap().1;
        assert_eq!(epic.name, "name".to_owned());
        assert_eq!(epic.description, "description".to_owned());
    }

    #[test]
    fn handle_action_should_handle_update_epic() {
        let dao = make_dao();
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.update_status = Box::new(|| Some(Status::InProgress));
        sut.set_prompts(prompts);

        sut.handle_action(Action::UpdateEpicStatus { epic_id })
            .unwrap();

        let db_state = dao.read_db().unwrap();
        assert_eq!(
            db_state.epics.get(&epic_id).unwrap().status,
            Status::InProgress
        );
    }

    #[test]
    fn handle_action_should_handle_delete_epic() {
        let dao = make_dao();
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.delete_epic = Box::new(|| true);
        sut.set_prompts(prompts);

        sut.handle_action(Action::DeleteEpic { epic_id }).unwrap();

        let db_state = dao.read_db().unwrap();
        assert_eq!(db_state.epics.len(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_story() {
        let dao = make_dao();
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.create_story = Box::new(|| Story::new("name".to_owned(), "description".to_owned()));
        sut.set_prompts(prompts);

        sut.handle_action(Action::CreateStory { epic_id }).unwrap();

        let db_state = dao.read_db().unwrap();
        assert_eq!(db_state.stories.len(), 1);

        let story = db_state.stories.into_iter().next().unwrap().1;
        assert_eq!(story.name, "name".to_owned());
        assert_eq!(story.description, "description".to_owned());
    }

    #[test]
    fn handle_action_should_handle_update_story() {
        let dao = make_dao();
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let story_id = dao
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .unwrap();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.update_status = Box::new(|| Some(Status::InProgress));
        sut.set_prompts(prompts);
        sut.handle_action(Action::UpdateStoryStatus { story_id })
            .unwrap();
        let db_state = dao.read_db().unwrap();
        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::InProgress
        );
    }

    #[test]
    fn handle_action_should_handle_delete_story() {
        let dao = make_dao();
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let story_id = dao
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .unwrap();
        let mut sut = Navigator::new(Rc::clone(&dao));
        let mut prompts = Prompts::new();
        prompts.delete_story = Box::new(|| true);
        sut.set_prompts(prompts);
        sut.handle_action(Action::DeleteStory { epic_id, story_id })
            .unwrap();
        let db_state = dao.read_db().unwrap();
        assert_eq!(db_state.stories.len(), 0);
    }
}
