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
    use super::*;

    use crate::rpn_controller::stack_controller::{Entered, StackMachine};

    #[test]
    fn it_pushes_to_the_stack() {
        let mut state = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 256.0,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let _ = state.push();

        let result = state.push();

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 256.0,
                y: Option::Some(256.0),
                rest: vec![256.0].into(),
            }),
        };

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
