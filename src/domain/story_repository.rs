use super::Story;

pub trait StoryRepository {
    fn get_id(&self) -> Result<u32, ()>;
    fn create(&self, story: Story) -> Result<(), ()>;
    fn update(&self, story: Story) -> Result<(), ()>;
    fn delete(&self, story_id: u32) -> Result<(), ()>;
    fn get(&self, story_id: u32) -> Result<Option<Story>, ()>;
}
