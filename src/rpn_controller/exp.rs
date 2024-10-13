use crate::rpn_controller::stack_controller::StackMachine;

use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

pub trait Exp {
    fn exp(&mut self) -> Result<&mut Self, CalcStateFailure>;
}

impl Exp for CalcState {
    fn exp(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let mut new_stack = self.into_entered()?;

        new_stack.x = new_stack.x.exp();

        self.stack = StackMachine::EnteredValue(new_stack);

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rpn_controller::stack_controller::Entered;

    #[test]
    fn it_exponentiates_zero() {
        let mut state = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 0.0,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let result = state.exp();

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 1.0,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
