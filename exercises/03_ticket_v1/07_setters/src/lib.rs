// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Ticket::validate_title(&title);
        Ticket::validate_status(&status);
        Ticket::validate_description(&description);
        Ticket {
            title,
            description,
            status,
        }
    }

    // Used the same validation implemented previously at section 02_validation

    fn validate_status(status: &String) {
        let possible_statuses = vec!["To-Do", "In Progress", "Done"];
        if !possible_statuses.contains(&status.trim()) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }

    fn validate_title(value: &String) {
        Ticket::validate_not_empty(value, "Title");
        Ticket::validate_max_size(value, 50, "Title");
    }

    fn validate_description(value: &String) {
        Ticket::validate_not_empty(value, "Description");
        Ticket::validate_max_size(value, 500, "Description");
    }

    fn validate_not_empty(value: &String, field: &str) {
        if value.is_empty() {
            panic!("{} cannot be empty", field);
        }
    }

    fn validate_max_size(value: &String, max_size: usize, field: &str) {
        if value.len() > max_size {
            panic!("{} cannot be longer than {} bytes", field, max_size);
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, value: String) {
        Ticket::validate_status(&value);
        self.status = value;
    }

    pub fn set_description(&mut self, value: String) {
        Ticket::validate_description(&value);
        self.description = value;
    }

    pub fn set_title(&mut self, value: String) {
        Ticket::validate_title(&value);
        self.title = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
