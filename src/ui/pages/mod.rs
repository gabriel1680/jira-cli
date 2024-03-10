use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

use crate::dao::JiraDAO;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
}

pub struct HomePage {
    pub dao: Rc<JiraDAO>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        let epics = self.dao.read_db()?.epics;
        for id in epics.keys().sorted() {
            let epic = &epics[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epics = self.dao.read_db()?.epics;
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {
                    if epics.contains_key(&epic_id) {
                        return Ok(Some(Action::NavigateToEpicDetail { epic_id }));
                    }
                }
                Ok(None)
            }
        }
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub dao: Rc<JiraDAO>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let dao_state = self.dao.read_db()?;
        let epic = dao_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        let id_col = get_column_string(&self.epic_id.to_string(), 11);
        let name_col = get_column_string(&epic.name, 32);
        let description_col = get_column_string(&epic.description, 32);
        let status_col = get_column_string(&epic.status.to_string(), 17);
        println!(
            "{} | {} | {} | {}",
            id_col, name_col, description_col, status_col
        );

        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &dao_state.stories;
        for id in stories.keys().sorted() {
            let story = &stories[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&story.name, 32);
            let status_col = get_column_string(&story.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let db_state = self.dao.read_db()?;
        let stories = db_state.stories;
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus {
                epic_id: self.epic_id,
            })),
            "d" => Ok(Some(Action::DeleteEpic {
                epic_id: self.epic_id,
            })),
            "c" => Ok(Some(Action::CreateStory {
                epic_id: self.epic_id,
            })),
            input => {
                if let Ok(story_id) = input.parse::<u32>() {
                    if stories.contains_key(&story_id) {
                        return Ok(Some(Action::NavigateToStoryDetail {
                            epic_id: self.epic_id,
                            story_id,
                        }));
                    }
                }
                Ok(None)
            }
        }
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub dao: Rc<JiraDAO>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let dao_state = self.dao.read_db()?;
        let story = dao_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        let id_col = get_column_string(&self.story_id.to_string(), 11);
        let name_col = get_column_string(&story.name, 32);
        let status_col = get_column_string(&story.status.to_string(), 17);
        println!("{} | {} | {}", id_col, name_col, status_col);

        println!();
        println!();

        println!("[p] previous | [u] update story | [d] delete story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus {
                story_id: self.story_id,
            })),
            "d" => Ok(Some(Action::DeleteStory {
                epic_id: self.epic_id,
                story_id: self.story_id,
            })),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::test_utils::MockDB;
    use crate::models::{Epic, Story};

    fn make_dao() -> Rc<JiraDAO> {
        let database = Box::new(MockDB::new());
        Rc::new(JiraDAO::new(database))
    }

    fn create_epic_and_story(dao: &JiraDAO) -> (u32, u32) {
        let epic_id = dao
            .create_epic(Epic::new("".to_owned(), "".to_owned()))
            .unwrap();
        let story_id = dao
            .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
            .unwrap();
        (epic_id, story_id)
    }

    mod home_page {
        use super::*;

        fn make_sut() -> HomePage {
            let dao = make_dao();
            HomePage { dao }
        }

        #[test]
        fn draw_page_should_not_throw_error() {
            let sut = make_sut();
            assert_eq!(sut.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let sut = make_sut();
            assert_eq!(sut.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let dao = make_dao();
            let epic = Epic::new("".to_owned(), "".to_owned());
            let epic_id = dao.create_epic(epic).unwrap();
            let page = HomePage { dao };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod epic_detail_page {
        use super::*;

        fn make_sut(with_epic: Option<()>) -> EpicDetail {
            let dao = make_dao();
            match with_epic {
                Some(()) => {
                    let epic_id = dao
                        .create_epic(Epic::new("".to_owned(), "".to_owned()))
                        .unwrap();
                    EpicDetail { epic_id, dao }
                }
                None => EpicDetail { epic_id: 999, dao },
            }
        }

        #[test]
        fn draw_page_should_not_throw_error() {
            let sut = make_sut(Some(()));
            assert_eq!(sut.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let sut = make_sut(Some(()));
            assert_eq!(sut.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let sut = make_sut(None);
            assert_eq!(sut.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let dao = make_dao();
            let epic_id = dao
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = dao
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();
            let sut = EpicDetail { epic_id, dao };
            let (p, u, d, c) = ("p", "u", "d", "c");
            let (
                invalid_story_id,
                junk_input,
                junk_input_with_valid_prefix,
                input_with_trailing_white_spaces,
            ) = ("999", "j983f2j", "p983f2j", "p\n");

            assert_eq!(
                sut.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                sut.handle_input(u).unwrap(),
                Some(Action::UpdateEpicStatus { epic_id: 1 })
            );
            assert_eq!(
                sut.handle_input(d).unwrap(),
                Some(Action::DeleteEpic { epic_id: 1 })
            );
            assert_eq!(
                sut.handle_input(c).unwrap(),
                Some(Action::CreateStory { epic_id: 1 })
            );
            assert_eq!(
                sut.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail {
                    epic_id: 1,
                    story_id: 2
                })
            );
            assert_eq!(sut.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(sut.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                sut.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                sut.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod story_detail_page {
        use super::*;

        fn make_sut() -> StoryDetail {
            let dao = make_dao();
            let (epic_id, story_id) = create_epic_and_story(&dao);
            StoryDetail {
                epic_id,
                story_id,
                dao,
            }
        }

        #[test]
        fn draw_page_should_not_throw_error() {
            let sut = make_sut();
            assert_eq!(sut.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let sut = make_sut();
            assert_eq!(sut.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let dao = make_dao();
            let epic_id = dao
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let _ = dao
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();
            let sut = StoryDetail {
                epic_id,
                story_id: 999,
                dao,
            };
            assert_eq!(sut.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let sut = make_sut();
            let story_id = sut.story_id;
            let epic_id = sut.epic_id;

            let (p, u, d) = ("p", "u", "d");
            let (junk_input, junk_input_with_valid_prefix, input_with_trailing_white_spaces) =
                ("j983f2j", "p983f2j", "p\n");
            let some_number = "1";

            assert_eq!(
                sut.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                sut.handle_input(u).unwrap(),
                Some(Action::UpdateStoryStatus { story_id })
            );
            assert_eq!(
                sut.handle_input(d).unwrap(),
                Some(Action::DeleteStory { epic_id, story_id })
            );
            assert_eq!(sut.handle_input(some_number).unwrap(), None);
            assert_eq!(sut.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                sut.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                sut.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }
}
