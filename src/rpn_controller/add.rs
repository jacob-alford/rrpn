use crate::rpn_controller::stack_controller::{Entered, StackMachine};

use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn add(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let new_x = self.reduce_binary(|x, y| x + y)?;

        let unshift_result = self.stack.unshift();

        self.stack = StackMachine::EnteredValue(Entered {
            x: new_x,
            y: unshift_result.new_y,
            rest: unshift_result.new_rest,
        });

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rpn_controller::stack_controller::Entered;

    #[test]
    fn it_adds_two_numbers() {
        let mut state = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 69.69,
                y: Option::Some(420.42),
                rest: vec![].into(),
            }),
        };

        let result = state.add();

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 490.11,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
