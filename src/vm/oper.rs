use std::fmt::{self, Display};

// Enum storing all possible instructions
#[derive(Clone, Copy, Debug)]
pub enum Oper {
    Number(isize),
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
    StdIn,
}

impl Oper {
    // Parse isntructions from String
    pub fn try_new(val: String) -> Result<Vec<Oper>, String> {
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

fn is_string_numeric(val: &str) -> bool {
    for character in val.chars() {
        if !character.is_numeric() {
            return false;
        }
    }
    true
}
