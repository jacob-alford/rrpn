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
            buffer: "0".into(),
            initial: true,
            y: Option::None,
            rest: VecDeque::from([]),
        }),
    });

    rsx! {
        h1 { "RPN" }
        button {
            onclick: move |_| {
                controller.write().push().unwrap();
            },
            "enter"
        }
        button {
            onclick: move |_| {
                controller.write().add().unwrap();
            },
            "add"
        }
        button {
            onclick: move |_| {
                controller.write().enter(1);
            },
            "1"
        }
        button {
            onclick: move |_| {
                controller.write().enter(2);
            },
            "2"
        }

        button {
            onclick: move |_| {
                controller.write().enter(3);
            },
            "3"
        }

        button {
            onclick: move |_| {
                controller.write().enter(4);
            },
            "4"
        }

        button {
            onclick: move |_| {
                controller.write().enter(5);
            },
            "5"
        }

        button {
            onclick: move |_| {
                controller.write().enter(6);
            },
            "6"
        }

        button {
            onclick: move |_| {
                controller.write().enter(7);
            },
            "7"
        }

        button {
            onclick: move |_| {
                controller.write().enter(8);
            },
            "8"
        }

        button {
            onclick: move |_| {
                controller.write().enter(9);
            },
            "9"
        }

        button {
            onclick: move |_| {
                controller.write().enter(0);
            },
            "0"
        }

        p { "{controller:?}" }
    }
}
