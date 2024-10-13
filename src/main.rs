#![allow(non_snake_case)]

use std::collections::VecDeque;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use rrpn::rpn_controller::stack_controller::{StackMachine, Typing};
use rrpn::rpn_controller::state_controller::CalcState;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut controller = use_signal(|| CalcState { 
        stack: StackMachine::EnteringValue(Typing {
            buffer: "1.0".into(),
            y: Option::None,
            rest: VecDeque::from([])
        })
    });

    rsx! {
        h1 { "RPN" }
        button { "don't click me" }
        button { 
            onclick: move |_| {
                controller.write().push().unwrap();
            }, 
            "enter" 
        }
        p { "{controller:?}" }
    }
}
