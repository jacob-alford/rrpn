use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn push(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let next_stack = self
            .stack
            .push()
            .map_err(|err| CalcStateFailure::ParseFailure(err))?;

        self.stack = next_stack;

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::rpn_controller::state_controller::{EnteredBuilder, TypingBuilder};

    #[test]
    fn it_pushes_to_the_stack() {
        let mut state = EnteredBuilder::empty().finalize(256.0);

        let _ = state.push();

        let result = state.push();

        let mut expected = TypingBuilder::empty()
            .add_buffer("256")
            .add_initial(true)
            .add_y(256.0)
            .add_rest(vec![256.0])
            .finalize();

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
