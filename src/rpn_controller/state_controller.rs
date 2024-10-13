use core::num::ParseFloatError;
use core::fmt;

use crate::rpn_controller::stack_controller::{Entered, StackMachine};

#[derive(Debug, PartialEq)]
pub struct CalcState {
    pub stack: StackMachine,
}

#[derive(Debug, PartialEq)]
pub enum CalcStateFailure {
    ParseFailure(ParseFloatError),
    EmptyArgs(String),
}

impl fmt::Display for CalcState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.stack {
            StackMachine::EnteringValue(typing) => write!(f, "{}", typing.buffer),
            StackMachine::EnteredValue(entered) => write!(f, "{}", entered.x)
        }
    }
}

impl CalcState {
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
