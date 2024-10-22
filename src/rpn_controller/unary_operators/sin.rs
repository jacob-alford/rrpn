use crate::rpn_controller::stack_controller::StackMachine;

use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn sin(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let mut new_stack = self.into_entered()?;

        new_stack.x = new_stack.x.sin();

        self.stack = StackMachine::EnteredValue(new_stack);

        Result::Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::rpn_controller::state_controller::EnteredBuilder;
    use core::f64::consts::PI;

    #[test]
    fn it_exponentiates_zero() {
        let mut state = EnteredBuilder::empty().finalize(PI / 2.0);

        let result = state.sin();

        let mut expected = EnteredBuilder::empty().finalize(1.0);

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
