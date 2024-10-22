use crate::rpn_controller::stack_controller::{Entered, StackMachine};
use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn swap(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let next = self.into_entered()?;

        let next_x = match next.y {
            Option::None => Result::Err(CalcStateFailure::EmptyArgs("Missing y".into())),
            Option::Some(next_x) => Result::Ok(next_x),
        }?;

        self.stack = StackMachine::EnteredValue(Entered {
            x: next_x,
            y: Option::Some(next.x),
            rest: next.rest,
        });

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rpn_controller::state_controller::{EnteredBuilder, TypingBuilder};

    #[test]
    fn it_errors_with_missing_y() {
        let mut state = TypingBuilder::empty().add_buffer("1").finalize();

        let result = state.swap();

        assert_eq!(
            result,
            Result::Err(CalcStateFailure::EmptyArgs("Missing y".into()))
        )
    }

    #[test]
    fn it_swaps_x_and_y_while_entering() {
        let mut state = TypingBuilder::empty().add_buffer("1").add_y(2.0).finalize();

        let result = state.swap();

        let mut expected = EnteredBuilder::empty().add_y(1.0).finalize(2.0);

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
