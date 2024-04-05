use crate::{Command, Method, Segment};
use std::sync::{Mutex, OnceLock};

pub fn translate_command(command: Command, index: usize) -> String {
    println!("Translating command: {:?}", command);
    match command {
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Constant,
            value,
        } => format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Static,
            value,
        } => format!("@STATIC.{value}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::This,
            value,
        } => format!(
            "@{}\nD=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::That,
            value,
        } => format!(
            "@{}\nD=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Argument,
            value,
        } => format!(
            "@{}\nD=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Local,
            value,
        } => format!(
            "@{}\nD=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Temp,
            value,
        } => format!(
            "@{}\nD=A\n@5\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Pointer,
            value,
        } => format!(
            "@{}\nD=A\n@3\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Static,
            value,
        } => format!("@SP\nAM=M-1\nD=M\n@STATIC.{}\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::This,
            value,
        } => format!(
            "@{}\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::That,
            value,
        } => format!(
            "@{}\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Argument,
            value,
        } => format!(
            "@{}\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Local,
            value,
        } => format!(
            "@{}\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Pointer,
            value,
        } => format!(
            "@{}\nD=A\n@3\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Temp,
            value,
        } => format!(
            "@{}\nD=A\n@5\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
            value
        ),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Constant,
            value: _value,
        } => panic!("Cannot pop to constant segment"),
        Command::Add => "@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D+M".to_string(),
        Command::Sub => "@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=M-D".to_string(),
        Command::Neg => "@SP\nA=M-1\nM=-M".to_string(),
        Command::Eq => {
            format!("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nD=M-D\nM=-1\n@eqTrue{index}\nD;JEQ\n@SP\nA=M-1\nM=0\n(eqTrue{index})\n")
        }
        Command::Gt => {
            format!("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nD=M-D\nM=-1\n@gtTrue{index}\nD;JGT\n@SP\nA=M-1\nM=0\n(gtTrue{index})\n")
        }
        Command::Lt => {
            format!("@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nD=M-D\nM=-1\n@ltTrue{index}\nD;JLT\n@SP\nA=M-1\nM=0\n(ltTrue{index})\n")
        }
        Command::Not => "@SP\nA=M-1\nM=!M\n".to_string(),
        Command::Or => "@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D|M\n".to_string(),
        Command::And => "@SP\nAM=M-1\nD=M\n@SP\nA=M-1\nM=D&M\n".to_string(),
        Command::IfGoTo { label } => format!("@{label}\n0;JMP\n").to_string(),
        Command::GoTo { label } => {
            format!("@SP\nAM=M-1\nD=M\nA=A-1\n@{label}\nD;JNE\n").to_string()
        }
        Command::Label { label } => format!("({label})\n").to_string(),
        Command::Return => format!("@LCL\nD=M\n@frame\nM=D\n@5\nA=D-A\nD=M\n@RET\nM=D\n@SP\nA=M-1\nD=M\n@ARG\nA=M\nM=D\n@ARG\nD=M\n@SP\nM=D+1\n@1\nD=A\n@frame\nA=M-D\nD=M\n@THAT\nM=D\n@2\nD=A\n@frame\nA=M-D\nD=M\n@THIS\nM=D\n@3\nD=A\n@frame\nA=M-D\nD=M\n@ARG\nM=D\n@4\nD=A\n@frame\nA=M-D\nD=M\n@LCL\nM=D\n@RET\nA=M\n0;JMP\n").to_string(),
        Command::Function { label, n } => make_function_string(label, n),
        Command::Call { label, n } => make_call_string(label, n)
    }
}

fn make_function_string(label: String, n: u16) -> String {
    let mut function_string = format!("({label})\n", label = label);
    for _ in 0..n {
        function_string.push_str(&translate_command(
            Command::PushPop {
                method: Method::Push,
                segment: Segment::Constant,
                value: 0,
            },
            0,
        ));
    }
    function_string
}

fn count() -> &'static Mutex<u16> {
    static COUNT: OnceLock<Mutex<u16>> = OnceLock::new();
    COUNT.get_or_init(|| Mutex::new(0))
}

fn add_one() {
    let mut data = count().lock().unwrap();
    *data += 1;
}

fn make_call_string(label: String, n: u16) -> String {
    let return_address_label = format!("return_address_{}", count().lock().unwrap());
    add_one();
    format!("@{return_address_label}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n@SP\nD=M\n@{n}\nD=D-A\n@5\nD=D-A\n@ARG\nM=D\n@SP\nD=M\n@LCL\nM=D\n@{label}\n0;JMP\n({return_address_label})\n")
}
