use std::collections::VecDeque;

use rrpn::rpn_controller::stack_controller::{StackMachine, Typing};
use rrpn::rpn_controller::state_controller::CalcState;

fn main() {
    let mut foo = CalcState {
        stack: StackMachine::EnteringValue(Typing {
            buffer: "1.0".into(),
            y: Option::None,
            rest: VecDeque::from([]),
        }),
    };

    foo.push().unwrap();

    foo.add().unwrap();

    foo.exp().unwrap();

    println!("{:#?}", foo);
}
