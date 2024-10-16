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
    use super::*;

    use crate::rpn_controller::stack_controller::{Entered, StackMachine};

    #[test]
    fn it_types_numbers_when_typing() {
        let a = true;
        let b = true;

        assert_eq!(&a, &b)
    }
}
