use anyhow::Result;
use itertools::Itertools;
use std::rc::Rc;

use crate::dao::JiraDAO;
use crate::ui::actions::Action;
use crate::ui::pages::page_helpers::get_column_string;

use super::page::Page;

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {

    use crate::{models::Epic, ui::pages::page_test_utils::make_dao};

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
        let sut = HomePage { dao };

        let valid_epic_id = epic_id.to_string();
        let (q, c) = ("q", "c");
        let (
            invalid_epic_id,
            junk_input,
            junk_input_with_valid_prefix,
            input_with_trailing_white_spaces,
        ) = ("999", "j983f2j", "q983f2j", "q\n");

        assert_eq!(sut.handle_input(q).unwrap(), Some(Action::Exit));
        assert_eq!(sut.handle_input(c).unwrap(), Some(Action::CreateEpic));
        assert_eq!(
            sut.handle_input(&valid_epic_id).unwrap(),
            Some(Action::NavigateToEpicDetail { epic_id: 1 })
        );
        assert_eq!(sut.handle_input(invalid_epic_id).unwrap(), None);
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
