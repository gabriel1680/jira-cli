use std::rc::Rc;

use crate::domain::{EpicRepository, Story, StoryRepository};

pub struct CreateStory {
    repository: Rc<dyn StoryRepository>,
    epic_repository: Rc<dyn EpicRepository>,
}

impl CreateStory {
    pub fn new(
        repository: Rc<dyn StoryRepository>,
        epic_repository: Rc<dyn EpicRepository>,
    ) -> Self {
        Self {
            repository,
            epic_repository,
        }
    }

    pub fn execute(&self, input: CreateStoryInput) -> Result<(), ()> {
        let id = self.repository.get_id()?;
        let story = Story::new(id, input.epic_id, input.name, input.description);
        let Some(mut epic) = self.epic_repository.get(input.epic_id)? else {
            return Err(());
        };
        epic.add_story(id);
        self.epic_repository.update(&epic)?;
        self.repository.create(story)?;
        Ok(())
    }
}

pub struct CreateStoryInput {
    pub name: String,
    pub description: String,
    pub epic_id: u32,
}
