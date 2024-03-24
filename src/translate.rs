use crate::{Command, Method, Segment};





pub fn translate_command(command: Command, index: usize) -> String {
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
        } => format!("@{}\nD=A\n@THIS\nA=M+D\n D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::That,
            value,
        } => format!("@{}\nD=A\n@THAT\nA=M+D\n D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Argument,
            value,
        } => format!("@{}\nD=A\n@ARG\nA=M+D\n D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Local,
            value,
        } => format!("@{}\nD=A\n@LCL\nA=M+D\n D=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Temp,
            value,
        } => format!("@{}\nD=A\n@5\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Pointer,
            value,
        } => format!("@{}\nD=A\n@3\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Static,
            value,
        } => format!("@SP\nAM=M-1\nD=M\n@STATIC.{}\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::This,
            value,
        } => format!("@{}\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::That,
            value,
        } => format!("@{}\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Argument,
            value,
        } => format!("@{}\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Local,
            value,
        } => format!("@{}\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Pointer,
            value,
        } => format!("@{}\nD=A\n@3\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
        Command::PushPop {
            method: Method::Pop,
            segment: Segment::Temp,
            value,
        } => format!("@{}\nD=A\n@5\nD=A+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", value),
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
    }
}

