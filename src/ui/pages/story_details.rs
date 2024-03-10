use anyhow::{anyhow, Result};
use std::rc::Rc;

use crate::models::Action;
use crate::{dao::JiraDAO, ui::pages::get_column_string};

use super::Page;

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
    use crate::{
        models::{Epic, Story},
        ui::{page_test_utils::make_dao, pages::epic_details::EpicDetail},
    };

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
