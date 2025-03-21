use core::panic;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //  You can find the needed panic messages in the tests.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        Ticket::validate_title(&title);
        Ticket::validate_status(&status);
        Ticket::validate_description(&description);
        Self {
            title,
            description,
            status,
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
