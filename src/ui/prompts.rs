use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("Epic Name:");
    let name = get_user_input();
    println!("Epic Description:");
    let description = get_user_input();
    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!("Story Name:");
    let name = get_user_input();
    println!("Story Description:");
    let description = get_user_input();
    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    draw_header("Are you sure you want to delete this story? [Y/n]: ");
    get_user_input().trim().eq("Y")
}

fn delete_story_prompt() -> bool {
    draw_header("Are you sure you want to delete this story? [Y/n]: ");
    get_user_input().trim().eq("Y")
}

fn update_status_prompt() -> Option<Status> {
    draw_header("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): ");
    let status = get_user_input().trim().parse::<u8>();
    if let Ok(status) = status {
        match status {
            1 => Some(Status::Open),
            2 => Some(Status::Closed),
            3 => Some(Status::Resolved),
            4 => Some(Status::InProgress),
            _ => None,
        };
    }
    None
}

fn draw_header(text: &str) {
    println!("----------------------------");
    println!("{}", text);
}
