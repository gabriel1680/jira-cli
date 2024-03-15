use super::{Status, StatusState};

pub struct Epic {
    id: u32,
    name: String,
    description: String,
    state: StatusState,
    stories: Vec<u32>,
}

impl Epic {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            state: StatusState::new(Status::Open),
            stories: vec![],
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
        assert_eq!(sut.state.get_status(), Status::Open);
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
