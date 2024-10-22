use crate::rpn_controller::stack_controller::{Entered, StackMachine};

use crate::rpn_controller::state_controller::{CalcState, CalcStateFailure};

impl CalcState {
    pub fn div(&mut self) -> Result<&mut Self, CalcStateFailure> {
        let new_x = self.reduce_binary(|x, y| x / y)?;

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
    use crate::rpn_controller::state_controller::EnteredBuilder;

    #[test]
    fn it_divides_two_numbers() {
        let mut state = EnteredBuilder::empty().add_y(420.42).finalize(69.69);

        let result = state.div();

        let mut expected = EnteredBuilder::empty().finalize(0.16576280861995146);

        assert_eq!(result, Result::Ok(&mut expected))
    }
}
