use std::collections::VecDeque;

pub type MemoryUnit = i128;

pub enum Oper {
    Number(MemoryUnit),
    Push,
    Pop,
    Cpy,
    StackCount,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    PushLineRegister,
    PopLineRegister,
    StdOut,
    StdIn,
}

impl Oper {
    pub fn from_string(s: String) -> Result<Oper, String> {
        if is_string_numeric(&s) {
            Ok(Oper::Number(s.parse().unwrap()))
        } else {
            match s.as_str() {
                "[" => Ok(Oper::Push),
                "]" => Ok(Oper::Pop),
                "@" => Ok(Oper::Cpy),
                "$" => Ok(Oper::StackCount),
                "+" => Ok(Oper::Add),
                "-" => Ok(Oper::Sub),
                "*" => Ok(Oper::Mul),
                "/" => Ok(Oper::Div),
                "%" => Ok(Oper::Mod),
                "<" => Ok(Oper::PushLineRegister),
                ">" => Ok(Oper::PopLineRegister),
                "!" => Ok(Oper::StdOut),
                "?" => Ok(Oper::StdIn),
                _ => Err(format!("String '{s:}' cannot be parsed as an operation.")),
            }
        }
    }
}

pub struct VirtualMachine {
    stack: VecDeque<MemoryUnit>,
    op_register: MemoryUnit,
    operation_register: MemoryUnit,
    instructions: Vec<Oper>,
}

fn is_string_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}
