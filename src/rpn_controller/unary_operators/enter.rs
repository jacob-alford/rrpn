use crate::rpn_controller::state_controller::CalcState;

impl CalcState {
    pub fn enter(&mut self, new_val: u8) -> &mut Self {
        let next_stack = self.stack.enter(new_val);

        self.stack = next_stack;

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::rpn_controller::state_controller::{EnteredBuilder, TypingBuilder};

    #[test]
    fn it_appends_numbers_when_typing() {
        let mut state = TypingBuilder::empty().add_buffer("1").finalize();

        let result = state.enter(9);

        let mut expected = TypingBuilder::empty().add_buffer("19").finalize();

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_pushes_and_appends_when_entered() {
        let mut state = EnteredBuilder::empty().finalize(1.0);

        let result = state.enter(9);

        let mut expected = TypingBuilder::empty().add_buffer("9").add_y(1.0).finalize();

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_does_not_append_consecutive_zeroes() {
        let mut state = TypingBuilder::empty().add_initial(true).finalize();

        let result = state.enter(0);

        let mut expected = TypingBuilder::default().finalize();

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_replaces_entered_when_first_typing() {
        let mut state = TypingBuilder::empty().add_initial(true).finalize();

        let result = state.enter(6);

        let mut expected = TypingBuilder::empty().add_buffer("6").finalize();

        assert_eq!(result, &mut expected)
    }
}
