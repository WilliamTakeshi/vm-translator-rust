use crate::{Command, Method, Segment};

pub fn translate_command(command: Command, index: usize) -> String {
    match command {
        Command::PushPop {
            method: Method::Push,
            segment: Segment::Constant,
            value,
        } => format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", value),
        Command::PushPop {
            method: _method,
            segment: _segment,
            value: _value,
        } => format!("aaa"),
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

