use super::{DomainError, Status};

pub struct Epic {
    id: u32,
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>,
}

impl Epic {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
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
                "Epic with status {} cannot be started",
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
                "Epic with status {} cannot be closed",
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
                "Epic with status {} cannot be resolved",
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
                "Epic with status {} cannot be opened",
                status
            ))),
        }
    }

    pub fn add_story(&mut self, story_id: u32) {
        if !self.stories.contains(&story_id) {
            self.stories.push(story_id);
        }
    }
}

mod epic_test_fixtures {
    use super::*;

    pub struct EpicBuilder {
        epic: Epic,
    }

    impl EpicBuilder {
        pub fn new(id: u32, name: String, description: String) -> Self {
            Self {
                epic: Epic::new(id, name, description),
            }
        }

        pub fn with_status(mut self, status: Status) -> Self {
            self.epic.status = status;
            self
        }

        pub fn with_stories(mut self, stories: Vec<u32>) -> Self {
            self.epic.stories = stories;
            self
        }

        pub fn build(self) -> Epic {
            self.epic
        }
    }
}

#[cfg(test)]
mod tests {
    use tests::epic_test_fixtures::EpicBuilder;

    use super::*;

    fn default_builder() -> EpicBuilder {
        EpicBuilder::new(1, "some".to_owned(), "desc".to_owned())
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

    #[test]
    fn add_story() {
        let mut sut = default_builder().build();
        sut.add_story(1);
        assert_eq!(sut.stories, vec![1]);
    }

    #[test]
    fn add_story_existent() {
        let mut sut = default_builder().with_stories(vec![1]).build();

        sut.add_story(1);
        assert_eq!(sut.stories, vec![1]);

        sut.add_story(2);
        assert_eq!(sut.stories, vec![1, 2]);
    }
}
