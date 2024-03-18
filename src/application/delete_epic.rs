use std::rc::Rc;

use crate::domain::{EpicRepository, StoryRepository};

struct RemoveEpic {
    epic_repository: Rc<dyn EpicRepository>,
    story_repository: Rc<dyn StoryRepository>,
}

impl RemoveEpic {
    pub fn new(
        epic_repository: Rc<dyn EpicRepository>,
        story_repository: Rc<dyn StoryRepository>,
    ) -> Self {
        Self {
            epic_repository,
            story_repository,
        }
    }

    pub fn execute(&self, input: RemoveEpicInput) -> Result<(), ()> {
        let Some(epic) = self.epic_repository.get(input.epic_id)? else {
            return Err(());
        };
        for story_id in epic.get_stories().iter() {
            self.story_repository.delete(*story_id)?;
        }
        self.epic_repository.delete(epic.id)?;
        Ok(())
    }
}

struct RemoveEpicInput {
    epic_id: u32,
}
