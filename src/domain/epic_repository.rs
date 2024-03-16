use super::Epic;

pub trait EpicRepository {
    fn get_id(&self) -> Result<u32, ()>;
    fn create(&self, epic: &Epic) -> Result<(), ()>;
    fn update(&self, epic: &Epic) -> Result<(), ()>;
    fn delete(&self, epic: &Epic) -> Result<(), ()>;
    fn get(&self, epic_id: u32) -> Result<Option<Epic>, ()>;
}
