use std::{
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};
use thiserror::Error;

#[derive(Clone)]
pub struct Program {
    pub code: Vec<i32>,
}

impl Program {
    pub fn execute(&mut self, input: &[i32]) -> Result<Vec<i32>, ExecutionError> {
        let mut program_counter: usize = 0;
        let mut output = Vec::new();
        let mut input_it = input.iter();
        loop {
            let Instruction {
                opcode,
                mut mode_flag,
            } = self.fetch_instruction(&mut program_counter)?;
            match opcode {
                Opcode::Arithmetic(op) => {
                    let lhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    let rhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    let target_pos =
                        self.fetch_positional_parameter(&mut program_counter, &mut mode_flag)?;

                    let result = match op {
                        ArithmeticOperation::Add => lhs + rhs,
                        ArithmeticOperation::Mul => lhs * rhs,
                    };

                    *self.get_mut(target_pos)? = result;
                }

                Opcode::Store => {
                    let pos =
                        self.fetch_positional_parameter(&mut program_counter, &mut mode_flag)?;
                    let inp = input_it
                        .next()
                        .ok_or(ExecutionError::UnexpectedEndOfInput)?;

                    *self.get_mut(pos)? = *inp;
                }

                Opcode::Jump(cond) => {
                    let lhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    let rhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;

                    let condition_satisfied = match cond {
                        JumpCondition::True => lhs != 0,
                        JumpCondition::False => lhs == 0,
                    };

                    if condition_satisfied {
                        program_counter = rhs.try_into()?;
                    }
                }

                Opcode::Compare(comp) => {
                    let lhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    let rhs = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    let target_pos =
                        self.fetch_positional_parameter(&mut program_counter, &mut mode_flag)?;

                    let comparison_fulfilled = match comp {
                        Comparison::LessThan => lhs < rhs,
                        Comparison::Equals => lhs == rhs,
                    };

                    *self.get_mut(target_pos)? = comparison_fulfilled.into();
                }

                Opcode::Print => {
                    let param = self.fetch_parameter(&mut program_counter, &mut mode_flag)?;
                    output.push(param);
                }

                Opcode::Exit => return Ok(output),
            }
        }
    }

    fn fetch_positional_parameter(
        &self,
        pos: &mut usize,
        mode_flag: &mut i32,
    ) -> Result<usize, ExecutionError> {
        let mode = Program::read_next_parameter_mode(mode_flag)?;
        match mode {
            ParameterMode::Immediate => Err(ExecutionError::InvalidImmediateParameter),
            ParameterMode::Position => self.fetch_position(pos),
        }
    }

    fn fetch_parameter(&self, pos: &mut usize, mode_flag: &mut i32) -> Result<i32, ExecutionError> {
        let mode = Program::read_next_parameter_mode(mode_flag)?;
        match mode {
            ParameterMode::Immediate => self.fetch_operand(pos),
            ParameterMode::Position => {
                let p = self.fetch_position(pos)?;
                self.get(p)
            }
        }
    }

    fn read_next_parameter_mode(mode_flag: &mut i32) -> Result<ParameterMode, ExecutionError> {
        let mode = *mode_flag % 10;
        *mode_flag /= 10;
        match mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(ExecutionError::UnknownParameterMode(mode)),
        }
    }

    fn fetch_instruction(&self, pos: &mut usize) -> Result<Instruction, ExecutionError> {
        let value = self.fetch_operand(pos)?;
        let opcode = Program::parse_opcode(value % 100)?;
        let mode_flag = value / 100;
        Ok(Instruction { opcode, mode_flag })
    }

    fn parse_opcode(opcode: i32) -> Result<Opcode, ExecutionError> {
        match opcode {
            1 => Ok(Opcode::Arithmetic(ArithmeticOperation::Add)),
            2 => Ok(Opcode::Arithmetic(ArithmeticOperation::Mul)),
            3 => Ok(Opcode::Store),
            4 => Ok(Opcode::Print),
            5 => Ok(Opcode::Jump(JumpCondition::True)),
            6 => Ok(Opcode::Jump(JumpCondition::False)),
            7 => Ok(Opcode::Compare(Comparison::LessThan)),
            8 => Ok(Opcode::Compare(Comparison::Equals)),
            99 => Ok(Opcode::Exit),
            _ => Err(ExecutionError::UnknownOpcode(opcode)),
        }
    }

    fn fetch_position(&self, pos: &mut usize) -> Result<usize, ExecutionError> {
        Ok(self.fetch_operand(pos)?.try_into()?)
    }

    fn fetch_operand(&self, pos: &mut usize) -> Result<i32, ExecutionError> {
        let result = self.get(*pos)?;
        *pos += 1;
        Ok(result)
    }

    fn get_mut(&mut self, pos: usize) -> Result<&mut i32, ExecutionError> {
        self.code.get_mut(pos).ok_or(ExecutionError::OutOfBounds)
    }

    fn get(&self, pos: usize) -> Result<i32, ExecutionError> {
        Ok(*self.code.get(pos).ok_or(ExecutionError::OutOfBounds)?)
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

enum ArithmeticOperation {
    Add,
    Mul,
}

enum JumpCondition {
    True,
    False,
}

enum Comparison {
    LessThan,
    Equals,
}

enum Opcode {
    Arithmetic(ArithmeticOperation),
    Store,
    Print,
    Jump(JumpCondition),
    Compare(Comparison),
    Exit,
}

struct Instruction {
    opcode: Opcode,
    mode_flag: i32,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Execution(#[from] ExecutionError),
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Unknown opcode {0}")]
    UnknownOpcode(i32),
    #[error("Attempted to read or write out of bounds")]
    OutOfBounds,
    #[error("Attempted to read some nonexistent input")]
    UnexpectedEndOfInput,
    #[error("Unknown parameter mode {0}")]
    UnknownParameterMode(i32),
    #[error("Received immediate mode parameter in an invalid position")]
    InvalidImmediateParameter,
    #[error("Conversion from int failed")]
    FromInt(#[from] TryFromIntError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    Number(#[from] ParseIntError),
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: Result<_, _> = s.split(',').map(str::parse).collect();
        Ok(Program { code: code? })
    }
}
