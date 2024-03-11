use super::{DomainError, Status};

pub struct Story {
    id: u32,
    epic_id: u32,
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>,
}

impl Story {
    pub fn new(id: u32, epic_id: u32, name: String, description: String) -> Self {
        Self {
            id,
            epic_id,
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }

    pub fn start(&mut self) -> Result<(), DomainError> {
        match &self.status {
            Status::Open | Status::InProgress => {
                self.status = Status::InProgress;
                Ok(())
            }
            status => Err(DomainError(format!(
                "Story with status {} cannot be started",
                status
            ))),
        }
    }

    pub fn close(&mut self) -> Result<(), DomainError> {
        match &self.status {
            Status::Open | Status::InProgress => {
                self.status = Status::Closed;
                Ok(())
            }
            status => Err(DomainError(format!(
                "Story with status {} cannot be closed",
                status
            ))),
        }
    }

    pub fn resolve(&mut self) -> Result<(), DomainError> {
        match &self.status {
            Status::Open | Status::InProgress => {
                self.status = Status::Resolved;
                Ok(())
            }
            status => Err(DomainError(format!(
                "Story with status {} cannot be resolved",
                status
            ))),
        }
    }

    pub fn open(&mut self) -> Result<(), DomainError> {
        match &self.status {
            Status::Closed | Status::Resolved => {
                self.status = Status::Open;
                Ok(())
            }
            status => Err(DomainError(format!(
                "Story with status {} cannot be opened",
                status
            ))),
        }
    }
}

mod story_test_fixtures {
    use super::*;

    pub struct StoryBuilder {
        story: Story,
    }

    impl StoryBuilder {
        pub fn new(id: u32, epic_id: u32, name: String, description: String) -> Self {
            Self {
                story: Story::new(id, epic_id, name, description),
            }
        }

        pub fn with_epic_id(mut self, epic_id: u32) -> Self {
            self.story.epic_id = epic_id;
            self
        }

        pub fn with_status(mut self, status: Status) -> Self {
            self.story.status = status;
            self
        }

        pub fn build(self) -> Story {
            self.story
        }
    }
}

#[cfg(test)]
mod tests {
    use tests::story_test_fixtures::StoryBuilder;

    use super::*;

    fn default_builder() -> StoryBuilder {
        StoryBuilder::new(1, 1, "some".to_owned(), "desc".to_owned())
    }

    #[test]
    fn new_should_start_with_open_status() {
        let sut = default_builder().build();
        assert_eq!(sut.status, Status::Open);
    }

    #[test]
    fn start_should_work() {
        let mut sut = default_builder().build();
        assert_eq!(sut.start().is_ok(), true);
        assert_eq!(sut.status, Status::InProgress);
    }

    #[test]
    fn start_should_fail() {
        let mut sut = default_builder().with_status(Status::Closed).build();
        assert_eq!(sut.start().is_err(), true);
    }

    #[test]
    fn close_should_work() {
        let mut sut = default_builder().build();
        assert_eq!(sut.close().is_ok(), true);
        assert_eq!(sut.status, Status::Closed);
    }

    #[test]
    fn close_should_fail() {
        let mut sut = default_builder().with_status(Status::Resolved).build();
        assert_eq!(sut.close().is_err(), true);
    }

    #[test]
    fn resolve_should_work() {
        let mut sut = default_builder().build();
        assert_eq!(sut.resolve().is_ok(), true);
        assert_eq!(sut.status, Status::Resolved);
    }

    #[test]
    fn resolve_should_fail() {
        let mut sut = default_builder().with_status(Status::Closed).build();
        assert_eq!(sut.start().is_err(), true);
    }
}
