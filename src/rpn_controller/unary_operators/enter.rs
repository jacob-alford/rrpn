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

    use crate::rpn_controller::stack_controller::{Entered, StackMachine, Typing};

    #[test]
    fn it_appends_numbers_when_typing() {
        let mut state = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "1".into(),
                initial: false,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let result = state.enter(9);

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "19".into(),
                initial: false,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_pushes_and_appends_when_entered() {
        let mut state = CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x: 1.0,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let result = state.enter(9);

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "9".into(),
                initial: true,
                y: Option::Some(1.0),
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_does_not_append_consecutive_zeroes() {
        let mut state = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "0".into(),
                initial: true,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let result = state.enter(0);

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "0".into(),
                initial: false,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, &mut expected)
    }

    #[test]
    fn it_replaces_entered_when_first_typing() {
        let mut state = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "0".into(),
                initial: true,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        let result = state.enter(6);

        let mut expected: CalcState = CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: "6".into(),
                initial: false,
                y: Option::None,
                rest: vec![].into(),
            }),
        };

        assert_eq!(result, &mut expected)
    }
}
