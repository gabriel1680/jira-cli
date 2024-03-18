use std::rc::Rc;

use crate::domain::{EpicRepository, StoryRepository};

struct DeleteStory {
    epic_repository: Rc<dyn EpicRepository>,
    story_repository: Rc<dyn StoryRepository>,
}

impl DeleteStory {
    pub fn new(
        epic_repository: Rc<dyn EpicRepository>,
        story_repository: Rc<dyn StoryRepository>,
    ) -> Self {
        Self {
            epic_repository,
            story_repository,
        }
    }

    pub fn execute(&self, input: DeleteStoryInput) -> Result<(), ()> {
        let Some(mut epic) = self.epic_repository.get(input.epic_id)? else {
            return Err(());
        };
        epic.remove_story(input.story_id);
        let Some(story) = self.story_repository.get(input.story_id)? else {
            return Err(());
        };
        self.story_repository.delete(input.story_id)?;
        self.epic_repository.update(&epic);
        Ok(())
    }
}

struct DeleteStoryInput {
    story_id: u32,
    epic_id: u32,
}
