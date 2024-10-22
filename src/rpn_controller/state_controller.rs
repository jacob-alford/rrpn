use core::fmt;
use core::num::ParseFloatError;
use std::collections::VecDeque;

use crate::rpn_controller::stack_controller::{Entered, StackMachine, Typing};

#[derive(Debug, PartialEq, Clone)]
pub enum AngleMeasure {
    Radians,
    Degrees,
}

#[derive(Debug, PartialEq)]
pub struct CalcState {
    pub stack: StackMachine,
    pub angle_measure: AngleMeasure,
}

impl CalcState {
    pub fn new_typing(args: TypingBuilder) -> CalcState {
        CalcState {
            stack: StackMachine::EnteringValue(Typing {
                buffer: match args.buffer {
                    Option::None => "0".into(),
                    Option::Some(b) => b.into(),
                },
                y: args.y,
                initial: args.initial.unwrap_or(false),
                rest: match args.rest {
                    Option::None => vec![].into(),
                    Option::Some(r) => r.into(),
                },
            }),
            angle_measure: args.angle_measure.unwrap_or(AngleMeasure::Degrees),
        }
    }

    pub fn new_entered(x: f64, args: EnteredBuilder) -> CalcState {
        CalcState {
            stack: StackMachine::EnteredValue(Entered {
                x,
                y: args.y,
                rest: match args.rest {
                    Option::None => vec![].into(),
                    Option::Some(r) => r.into(),
                },
            }),
            angle_measure: match args.angle_measure {
                Option::None => AngleMeasure::Degrees,
                Option::Some(am) => am,
            },
        }
    }

    pub fn into_entered(&mut self) -> Result<Entered, CalcStateFailure> {
        match &self.stack {
            StackMachine::EnteringValue(typing) => typing
                .into_entered()
                .map_err(|err| CalcStateFailure::ParseFailure(err)),
            StackMachine::EnteredValue(entered) => Result::Ok(entered.clone()),
        }
    }

    pub fn reduce_binary<F>(&mut self, f: F) -> Result<f64, CalcStateFailure>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let new_stack = self.into_entered()?;

        match new_stack.y {
            Option::None => Result::Err(CalcStateFailure::EmptyArgs("Missing Y".into())),
            Option::Some(y_val) => Result::Ok(f(new_stack.x, y_val)),
        }
    }
}

impl fmt::Display for CalcState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.stack {
            StackMachine::EnteringValue(typing) => write!(f, "{}", typing.buffer),
            StackMachine::EnteredValue(entered) => write!(f, "{}", entered.x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CalcStateFailure {
    ParseFailure(ParseFloatError),
    EmptyArgs(String),
    AlreadyDeciated(String),
}

#[derive(Default, Clone)]
pub struct EnteredBuilder {
    pub y: Option<f64>,
    pub rest: Option<VecDeque<f64>>,
    pub angle_measure: Option<AngleMeasure>,
}

impl EnteredBuilder {
    pub fn empty() -> EnteredBuilder {
        EnteredBuilder::default()
    }

    pub fn add_y(&mut self, y: f64) -> &EnteredBuilder {
        self.y = Option::Some(y);
        self
    }

    pub fn add_rest<V: Clone + Into<VecDeque<f64>>>(&mut self, rest: V) -> &EnteredBuilder {
        self.rest = Option::Some(rest.clone().into());
        self
    }

    pub fn add_angle_measure(&mut self, angle_measure: AngleMeasure) -> &EnteredBuilder {
        self.angle_measure = Option::Some(angle_measure);
        self
    }

    pub fn finalize(&self, x: f64) -> CalcState {
        CalcState::new_entered(x, self.clone())
    }
}

#[derive(Default)]
pub struct TypingBuilder {
    pub buffer: Option<String>,
    pub y: Option<f64>,
    pub initial: Option<bool>,
    pub rest: Option<VecDeque<f64>>,
    pub angle_measure: Option<AngleMeasure>,
}

impl TypingBuilder {
    pub fn empty() -> TypingBuilder {
        TypingBuilder::default()
    }

    pub fn add_buffer<S: Into<String>>(mut self, buffer: S) -> TypingBuilder {
        self.buffer = Option::Some(buffer.into());
        self
    }

    pub fn add_y(mut self, y: f64) -> TypingBuilder {
        self.y = Option::Some(y);
        self
    }

    pub fn add_initial(mut self, initial: bool) -> TypingBuilder {
        self.initial = Option::Some(initial);
        self
    }

    pub fn add_rest<V: Clone + Into<VecDeque<f64>>>(mut self, rest: V) -> TypingBuilder {
        self.rest = Option::Some(rest.clone().into());
        self
    }

    pub fn add_angle_measure(mut self, angle_measure: AngleMeasure) -> TypingBuilder {
        self.angle_measure = Option::Some(angle_measure);
        self
    }

    pub fn finalize(self) -> CalcState {
        CalcState::new_typing(self)
    }
}
