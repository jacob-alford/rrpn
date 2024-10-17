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
    pub initial: bool,
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

        Result::Ok(StackMachine::EnteringValue(Typing {
            buffer: entered_value.x.to_string(),
            initial: true,
            y: Option::Some(entered_value.x),
            rest: entered_value.rest,
        }))
    }

    pub fn enter(&self, new_value: u8) -> Self {
        match self {
            StackMachine::EnteringValue(typing) => {
                // Will replace cloned buffer with new value is newly entered
                if typing.initial {
                    StackMachine::EnteringValue(Typing {
                        buffer: new_value.to_string().into(),
                        y: typing.y,
                        initial: false,
                        rest: typing.rest.clone(),
                    })
                } else {
                    // Does not concat leading zeros
                    if typing.buffer == "0" {
                        return StackMachine::EnteringValue(Typing {
                            buffer: new_value.to_string().into(),
                            y: typing.y,
                            initial: false,
                            rest: typing.rest.clone(),
                        });
                    // Types new value
                    } else {
                        let mut typing_value = typing.buffer.clone();

                        typing_value.push_str(&new_value.to_string());

                        return StackMachine::EnteringValue(Typing {
                            buffer: typing_value.into(),
                            y: typing.y,
                            initial: false,
                            rest: typing.rest.clone(),
                        });
                    }
                }
            }
            // Converts to typing and pushes value to the stack, replicates value for x
            StackMachine::EnteredValue(entered) => {
                let typing_value = new_value.to_string();

                let mut new_rest = entered.rest.clone();

                match entered.y {
                    Option::None => (),
                    Option::Some(new_y) => new_rest.push_front(new_y),
                };

                StackMachine::EnteringValue(Typing {
                    buffer: typing_value.into(),
                    y: Option::Some(entered.x),
                    initial: true,
                    rest: new_rest,
                })
            }
        }
    }
}
