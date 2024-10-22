use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn enter_dec(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let next_stack = self
            .stack
            .enter_dec()
            .map_err(|err| CalcStateFailure::AlreadyDeciated(err))?;

        self.stack = next_stack;

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rpn_controller::state_controller::{EnteredBuilder, TypingBuilder};

    #[test]
    fn it_adds_a_decimal_to_new_initiants() {
        let mut state = TypingBuilder::empty()
            .add_buffer("100.1234")
            .add_initial(true)
            .finalize();

        let result = state.enter_dec();

        let mut expected = TypingBuilder::empty().add_buffer("0.").finalize();

        assert_eq!(result, Result::Ok(&mut expected))
    }

    #[test]
    fn it_errors_if_decimal_already_present() {
        let mut state = TypingBuilder::empty().add_buffer("100.1234").finalize();

        let result = state.enter_dec();

        assert_eq!(
            result,
            Result::Err(CalcStateFailure::AlreadyDeciated(
                "Already a decimal".into()
            ))
        )
    }

    #[test]
    fn it_adds_a_decimal() {
        let mut state = TypingBuilder::empty().add_buffer("100").finalize();

        let result = state.enter_dec();

        let mut expected = TypingBuilder::empty().add_buffer("100.").finalize();

        assert_eq!(result, Result::Ok(&mut expected))
    }

    #[test]
    fn it_converts_to_typing() {
        let mut state = EnteredBuilder::empty().add_y(101.0).finalize(100.0);

        let result = state.enter_dec();

        let mut expected = TypingBuilder::empty()
            .add_buffer("0.")
            .add_y(100.0)
            .add_rest(vec![101.0])
            .finalize();

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
