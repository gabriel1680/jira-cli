use crate::domain::{Epic, EpicRepository};

pub struct CreateEpic {
    repository: Box<dyn EpicRepository>,
}

impl CreateEpic {
    pub fn new(repository: Box<dyn EpicRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, input: CreateEpicInput) -> Result<(), ()> {
        let id = self.repository.get_id()?;
        let epic = Epic::new(id, input.name, input.description);
        self.repository.create(&epic)?;
        Ok(())
    }
}

pub struct CreateEpicInput {
    pub name: String,
    pub description: String,
}
