use std::rc::Rc;

use dao::JiraDAO;
use json_file_database_adapter::JSONFileJiraDAOAdapter;
use navigator::Navigator;
use ui::get_user_input;

use crate::ui::wait_for_key_press;

mod dao;
mod json_file_database_adapter;
mod models;
mod navigator;
mod ui;
mod application;
mod domain;

fn main() {
    let database_adapter = JSONFileJiraDAOAdapter {
        path: "./data/db.json".to_owned(),
    };
    let dao = JiraDAO::new(Box::new(database_adapter));
    let mut navigator = Navigator::new(Rc::new(dao));

    loop {
        clearscreen::clear().unwrap();
        let page = match navigator.get_current_page() {
            Some(page) => page,
            None => break,
        };
        if let Err(error) = page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
            break;
        }
        let input = get_user_input();
        match page.handle_input(&input) {
            Err(error) => {
                println!(
                    "Error getting user input: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }
            Ok(action) => {
                if let Some(action) = action {
                    if let Err(error) = navigator.handle_action(action) {
                        println!("Error handling processing user input: {}\nPress any key to continue...", error);
                        wait_for_key_press();
                    }
                }
            }
        }
    }
}
