use std::{
    collections::VecDeque,
    fmt::{self, Display},
};

pub type MemoryUnit = i128;

#[derive(Clone, Copy, Debug)]
pub enum Oper {
    Number(MemoryUnit),
    Pop,
    Cpy,
    StackCount,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    PushProgramCounter,
    PopProgramCounter,
    // StdOut,
    StdIn,
}

impl Oper {
    fn load_oper_vec_from_string(val: String) -> Result<Vec<Oper>, String> {
        let mut out = vec![];

        for c in val
            .split_whitespace()
            .into_iter()
            .collect::<String>()
            .chars()
        {
            if !c.is_whitespace() {
                out.push(Oper::try_from(c.to_string())?)
            }
        }

        Ok(out)
    }
}

impl TryFrom<String> for Oper {
    type Error = String;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if is_string_numeric(&val) {
            Ok(Oper::Number(val.parse().unwrap()))
        } else {
            match val.as_str() {
                "!" => Ok(Oper::Pop),
                "@" => Ok(Oper::Cpy),
                "$" => Ok(Oper::StackCount),
                "+" => Ok(Oper::Add),
                "-" => Ok(Oper::Sub),
                "*" => Ok(Oper::Mul),
                "/" => Ok(Oper::Div),
                "%" => Ok(Oper::Mod),
                "<" => Ok(Oper::PushProgramCounter),
                ">" => Ok(Oper::PopProgramCounter),
                // "!" => Ok(Oper::StdOut),
                "?" => Ok(Oper::StdIn),
                _ => Err(format!("String '{val:}' cannot be parsed as an operation.")),
            }
        }
    }
}

impl Display for Oper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Oper::Number(num) => write!(f, "NUM {num:}"),
            Oper::Pop => write!(f, "POP"),
            Oper::Cpy => write!(f, "CPY"),
            Oper::StackCount => write!(f, "STACK_COUNT"),
            Oper::Add => write!(f, "ADD"),
            Oper::Sub => write!(f, "SUB"),
            Oper::Mul => write!(f, "MUL"),
            Oper::Div => write!(f, "DIV"),
            Oper::Mod => write!(f, "MOD"),
            Oper::PushProgramCounter => write!(f, "PUSH_PROGRAM_COUNTER"),
            Oper::PopProgramCounter => write!(f, "POP_PROGRAM_COUNTER"),
            Oper::StdIn => write!(f, "STD_IN"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VirtualMachine {
    stack: VecDeque<MemoryUnit>,
    program_counter: usize,
    instructions: Vec<Oper>,
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
        write!(f, "=================================")?;
        Ok(())
    }
}

impl VirtualMachine {
    pub fn new(instructions: String) -> Result<Self, String> {
        Ok(Self {
            stack: VecDeque::new(),
            program_counter: 0,
            instructions: Oper::load_oper_vec_from_string(instructions)?,
        })
    }

    pub fn step(&mut self) -> bool {
        if self.program_counter >= self.instructions.len() {
            false
        } else {
            if let Err(err) = self.exec_instruction() {
                println!("Error: {err:}");
                false
            } else {
                true
            }
        }
    }

    fn exec_instruction(&mut self) -> Result<(), String> {
        let current_instruction = self.instructions[self.program_counter];
        match current_instruction {
            Oper::Number(num) => self.stack.push_back(num),
            Oper::Pop => {
                if let Some(popped) = self.stack.pop_back() {
                    println!("{:?}", popped)
                } else {
                    Err("Failed popping from stack".to_string())?
                }
            }
            Oper::Cpy => {
                if let Some(popped) = self.stack.pop_back() {
                    if popped >= 1 {
                        if let Some(to_copy) = self.stack.pop_back() {
                            for _ in 0..(popped + 1) {
                                self.stack.push_back(to_copy)
                            }
                        } else {
                            Err("Failed popping from stack when copying")?
                        }
                    } else {
                        Err("Failed cloning a value less the 1 times")?
                    }
                } else {
                    Err("Failed popping from stack when copying")?
                }
            }
            Oper::StackCount => self.stack.push_back(self.stack.len() as i128),
            Oper::Add => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 + num2)
                    } else {
                        Err("Failed popping value 2 from stack when adding")?
                    }
                } else {
                    Err("Failed popping value 1 from stack when adding")?
                }
            }
            Oper::Sub => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 - num2)
                    } else {
                        Err("Failed popping value 2 from stack when subbing")?
                    }
                } else {
                    Err("Failed popping value 1 from stack when subbing")?
                }
            }
            Oper::Mul => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        self.stack.push_back(num1 * num2)
                    } else {
                        Err("Failed popping value 2 from stack when multipliing")?
                    }
                } else {
                    Err("Failed popping value 1 from stack when multipliing")?
                }
            }
            Oper::Div => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        if num2 != 0 {
                            self.stack.push_back(num1 / num2)
                        } else {
                            Err("Division by 0 error")?
                        }
                    } else {
                        Err("Failed popping value 2 from stack when dividing")?
                    }
                } else {
                    Err("Failed popping value 1 from stack when dividing")?
                }
            }
            Oper::Mod => {
                if let Some(num1) = self.stack.pop_back() {
                    if let Some(num2) = self.stack.pop_back() {
                        if num2 != 0 {
                            self.stack.push_back(num1 % num2)
                        } else {
                            Err("Modulo by 0 error")?
                        }
                    } else {
                        Err("Failed popping value 2 from stack when moduloing")?
                    }
                } else {
                    Err("Failed popping value 1 from stack when moduloing")?
                }
            }
            Oper::PushProgramCounter => self.stack.push_back(self.program_counter as i128),
            Oper::PopProgramCounter => todo!(),
            Oper::StdIn => todo!(),
        }

        if !matches!(current_instruction, Oper::PushProgramCounter) {
            self.program_counter += 1;
        }

        Ok(())
    }
}

fn is_string_numeric(val: &str) -> bool {
    for character in val.chars() {
        if !character.is_numeric() {
            return false;
        }
    }
    true
}
