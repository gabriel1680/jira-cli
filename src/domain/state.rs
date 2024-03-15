use super::{DomainError, Status};

pub struct StatusState {
    status: Status,
}

impl StatusState {
    pub fn new(status: Status) -> Self {
        Self { status }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_should_work() {
        let mut sut = StatusState::new(Status::Open);
        assert_eq!(sut.start().is_ok(), true);
        assert_eq!(sut.get_status(), Status::InProgress);
    }

    #[test]
    fn start_should_fail() {
        let mut sut = StatusState::new(Status::Closed);
        assert_eq!(sut.start().is_err(), true);
    }

    #[test]
    fn close_should_work() {
        let mut sut = StatusState::new(Status::Open);
        assert_eq!(sut.close().is_ok(), true);
        assert_eq!(sut.get_status(), Status::Closed);
    }

    #[test]
    fn close_should_fail() {
        let mut sut = StatusState::new(Status::Resolved);
        assert_eq!(sut.close().is_err(), true);
    }

    #[test]
    fn resolve_should_work() {
        let mut sut = StatusState::new(Status::Open);
        assert_eq!(sut.resolve().is_ok(), true);
        assert_eq!(sut.get_status(), Status::Resolved);
    }

    #[test]
    fn resolve_should_fail() {
        let mut sut = StatusState::new(Status::Closed);
        assert_eq!(sut.start().is_err(), true);
    }
}
