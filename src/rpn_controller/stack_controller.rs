use core::num::ParseFloatError;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Entered {
    pub x: f64,
    pub y: Option<f64>,
    pub rest: VecDeque<f64>,
}

#[derive(Debug, PartialEq)]
pub struct Typing {
    pub buffer: String,
    pub y: Option<f64>,
    pub rest: VecDeque<f64>,
}

impl Typing {
    pub fn into_entered(&self) -> Result<Entered, ParseFloatError> {
        if self.buffer == "" {
            return Result::Ok(Entered {
                x: 0.0,
                y: self.y,
                rest: self.rest.clone(),
            });
        } else {
            let parsed = self.buffer.parse()?;
            return Result::Ok(Entered {
                x: parsed,
                y: self.y,
                rest: self.rest.clone(),
            });
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StackMachine {
    EnteringValue(Typing),
    EnteredValue(Entered),
}

pub struct UnshiftResult {
    pub new_y: Option<f64>,
    pub new_rest: VecDeque<f64>,
}

impl StackMachine {
    pub fn unshift(&self) -> UnshiftResult {
        let mut rest = match self {
            StackMachine::EnteringValue(typing) => typing.rest.clone(),
            StackMachine::EnteredValue(entered) => entered.rest.clone(),
        };
        match rest.pop_front() {
            Option::None => UnshiftResult {
                new_y: Option::None,
                new_rest: VecDeque::from([]),
            },
            Option::Some(new_y) => UnshiftResult {
                new_y: Option::Some(new_y),
                new_rest: rest,
            },
        }
    }

    pub fn push(&self) -> Result<Self, ParseFloatError> {
        let mut entered_value: Entered = match self {
            StackMachine::EnteringValue(typing) => typing.into_entered(),
            StackMachine::EnteredValue(entered) => Result::Ok(entered.clone()),
        }?;

        match entered_value.y {
            Option::None => (),
            Option::Some(y) => entered_value.rest.push_front(y),
        }

        Result::Ok(StackMachine::EnteredValue(Entered {
            x: entered_value.x,
            y: Option::Some(entered_value.x),
            rest: entered_value.rest,
        }))
    }
}
