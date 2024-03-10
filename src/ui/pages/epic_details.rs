use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::rc::Rc;

use crate::models::Action;
use crate::{dao::JiraDAO, ui::pages::get_column_string};

use super::Page;

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

#[cfg(test)]
mod tests {
    use crate::{
        models::{Epic, Story},
        ui::{
            page_test_utils::{create_epic_and_story, make_dao},
            pages::story_details::StoryDetail,
        },
    };

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
