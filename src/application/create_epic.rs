use crate::domain::Epic;

pub struct CreateEpic {
    // repository: Box<dyn EpicRepository>,
}

pub trait EpicRepository {
    fn create(epic: Epic) -> Result<(), ()>;
}
