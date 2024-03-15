use super::{Status, StatusState};

pub struct Story {
    id: u32,
    epic_id: u32,
    name: String,
    description: String,
    state: StatusState,
    stories: Vec<u32>,
}

impl Story {
    pub fn new(id: u32, epic_id: u32, name: String, description: String) -> Self {
        Self {
            id,
            epic_id,
            name,
            description,
            state: StatusState::new(Status::Open),
            stories: vec![],
        }
    }
}

mod story_test_fixtures {
    use super::*;

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
        assert_eq!(sut.state.get_status(), Status::Open);
    }
}
