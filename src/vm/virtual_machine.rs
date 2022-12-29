use std::{
    collections::VecDeque,
    fmt::{self, Display},
    io::{self, Write},
};

use colored::Colorize;

use super::Oper;

// Enum storing possible runtime errors
#[derive(Clone, Debug)]
enum ExecError {
    StackPopError,
    DivisionByZeroError,
    StackOverflow,
    STDInError,
}

// Struct storing language runtime state
#[derive(Clone, Debug)]
pub struct VirtualMachine {
    stack: VecDeque<isize>,
    program_counter: isize,
    instructions: Vec<Oper>,
    endstate: bool,
}

impl Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "===== Virtual Machine State =====")?;
        writeln!(f, "Program counter: {:}", self.program_counter)?;
        writeln!(f, "Stack: {:?}", self.stack)?;
        writeln!(f, "\nInstructions")?;
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "{:}\t{:}", i, instruction)?;
        }
        Ok(())
    }
}

impl VirtualMachine {
    // Initialization of the vm
    pub fn try_new(instructions: String) -> Result<Self, String> {
        Ok(Self {
            stack: VecDeque::new(),
            program_counter: 0,
            instructions: Oper::try_new(instructions)?,
            endstate: false,
        })
    }

    // Run an execution cycle
    pub fn step(&mut self) {
        if (self.program_counter as usize) < self.instructions.len() {
            if let Err(err) = self.exec_instruction() {
                println!("{}", format!("Error: {err:?}").red());
                self.endstate = true;
            }
        } else {
            self.endstate = true;
        }
    }

    fn exec_instruction(&mut self) -> Result<(), ExecError> {
        let current_instruction = self.instructions[self.program_counter as usize];
        if !matches!(current_instruction, Oper::PopProgramCounter) {
            self.program_counter += 1;
        }

        match current_instruction {
            Oper::Number(num) => self.stack.push_back(num),
            Oper::Pop => {
                if let Some(popped) = self.stack.pop_back() {
                    println!("{}", format!("Output: {popped:?}").green().bold());
                } else {
                    Err(ExecError::StackPopError)?;
                }
            }
            Oper::Cpy => {
                if let Some(popped) = self.stack.pop_back() {
                    if popped >= 1 {
                        if let Some(to_copy) = self.stack.pop_back() {
                            for _ in 0..(popped + 1) {
                                self.stack.push_back(to_copy)
                            }
                            return Ok(());
                        }
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::StackCount => {
                if let Ok(count_isize) = self.stack.len().try_into() {
                    self.stack.push_back(count_isize);
                } else {
                    Err(ExecError::StackOverflow)?;
                }
            }
            Oper::Add => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 + num2);
                        return Ok(());
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::Sub => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 - num2);
                        return Ok(());
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::Mul => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 * num2);
                        return Ok(());
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::Div => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        if num2 != 0 {
                            self.stack.push_back(num1 / num2);
                            return Ok(());
                        } else {
                            Err(ExecError::DivisionByZeroError)?;
                        }
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::Mod => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        if num2 != 0 {
                            self.stack.push_back(num1 % num2);
                            return Ok(());
                        } else {
                            Err(ExecError::DivisionByZeroError)?;
                        }
                    }
                }
                Err(ExecError::StackPopError)?;
            }
            Oper::PushProgramCounter => self.stack.push_back(self.program_counter),
            Oper::PopProgramCounter => {
                if let Some(popped) = self.stack.pop_back() {
                    self.program_counter = popped
                } else {
                    Err(ExecError::StackPopError)?;
                }
            }
            Oper::StdIn => self.stack.push_back({
                print!("{}", format!("Enter input: ").bold());
                io::stdout()
                    .flush()
                    .expect(&format!("Failed to flush StdOut").red());

                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect(&format!("failed to read from StdIn").red());

                match input_text.trim().parse::<isize>() {
                    Ok(i) => i,
                    Err(_) => Err(ExecError::STDInError)?,
                }
            }),
        }

        Ok(())
    }

    pub fn endstate(&self) -> &bool {
        &self.endstate
    }
}
