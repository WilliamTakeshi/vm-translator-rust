use anyhow::Context;
use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, value};
use nom::IResult;
use std::fs::File;
use std::io::prelude::*;

mod translate;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long)]
    input: String,

    /// Number of the output file
    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut new_file = File::create(args.output)?;

    let foo = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| parse_line(line))
        .enumerate()
        .map(|(n, command)| translate::translate_command(command.unwrap(), n))
        .map(|line| new_file.write_all(format!("{}\n", line).as_bytes()))
        .collect::<Vec<_>>();

    println!("{:?}", foo);

    Ok(())
}

fn parse_line(input: &str) -> anyhow::Result<Command> {
    if input.starts_with("push") || input.starts_with("pop") {
        return parse_push_pop(input);
    } else if input.starts_with("add") {
        return Ok(Command::Add);
    } else if input.starts_with("sub") {
        return Ok(Command::Sub);
    } else if input.starts_with("neg") {
        return Ok(Command::Neg);
    } else if input.starts_with("eq") {
        return Ok(Command::Eq);
    } else if input.starts_with("gt") {
        return Ok(Command::Gt);
    } else if input.starts_with("lt") {
        return Ok(Command::Lt);
    } else if input.starts_with("and") {
        return Ok(Command::And);
    } else if input.starts_with("or") {
        return Ok(Command::Or);
    } else if input.starts_with("not") {
        return Ok(Command::Not);
    } else {
        return Err(anyhow::anyhow!("Invalid command"));
    }
}

fn parse_push_pop(input: &str) -> anyhow::Result<Command> {
    let mut parser_u16 = map_res(digit1::<&str, nom::error::Error<&str>>, |s: &str| {
        s.parse::<u16>()
    });

    let (line, method) = parse_push_pop_command(input)
        .map_err(|e| e.to_owned())
        .context("Failed to parse method")?;
    let (line, segment) = parse_segment(line)
        .map_err(|e| e.to_owned())
        .context("Failed to parse memory segment")?;
    let (_, value) = parser_u16(line)
        .map_err(|e| e.to_owned())
        .context("Failed to parse memory segment")?;
    Ok(Command::PushPop {
        method,
        segment,
        value,
    })
}

#[derive(Debug, Clone)]
enum Command {
    PushPop {
        method: Method,
        segment: Segment,
        value: u16,
    },
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    // Branching {
    //     method: Method,
    //     label: String,
    // },
    // Function {
    //     method: Method,
    //     label: String,
    //     n: u16,
    // },
    // Return,
}

#[derive(Debug, Clone)]
enum Method {
    Push,
    Pull,
}

fn parse_push_pop_command(input: &str) -> IResult<&str, Method> {
    alt((
        value(Method::Push, tag("push ")),
        value(Method::Pull, tag("pull ")),
    ))(input)
}

#[derive(Debug, Clone)]
enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

fn parse_segment(input: &str) -> IResult<&str, Segment> {
    alt((
        value(Segment::Local, tag("local ")),
        value(Segment::Argument, tag("argument ")),
        value(Segment::This, tag("this ")),
        value(Segment::That, tag("that ")),
        value(Segment::Constant, tag("constant ")),
        value(Segment::Static, tag("static ")),
        value(Segment::Pointer, tag("pointer ")),
        value(Segment::Temp, tag("temp ")),
    ))(input)
}

// #[cfg(test)]
// mod tests {
//     // use super::*;
//     use nom::combinator::value;
//     use nom::character::complete::alpha1;

//     #[test]
//     fn test_main() {

//         let mut parser = value(1234, alpha1);

//         assert_eq!(parser("abcd"), Ok(("", 1234)));
//     }
// }
