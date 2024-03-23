use anyhow::Context;
use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, value};
use nom::IResult;
use std::fs::File;
use std::io::prelude::*;

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
    // let mut new_file = File::create(args.output)?;

    let foo = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    println!("{:?}", foo);

    Ok(())
}

fn parse_line(input: &str) -> anyhow::Result<Command> {
    let mut parser_u16 = map_res(digit1::<&str, nom::error::Error<&str>>, |s: &str| {
        s.parse::<u16>()
    });

    let (line, method) = parse_command(input)
        .map_err(|e| e.to_owned())
        .context("Failed to parse method")?;
    let (line, segment) = parse_segment(line)
        .map_err(|e| e.to_owned())
        .context("Failed to parse memory segment")?;
    let (line, value) = parser_u16(line)
        .map_err(|e| e.to_owned())
        .context("Failed to parse memory segment")?;
    Ok(
        Command {
            method,
            segment,
            value,
        }
    )
}

#[derive(Debug)]
pub struct Command {
    method: Method,
    segment: Segment,
    value: u16,
}

#[derive(Debug, Clone)]
enum Method {
    Push,
    Pull,
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

fn parse_command(input: &str) -> IResult<&str, Method> {
    alt((
        value(Method::Push, tag("push ")),
        value(Method::Pull, tag("pull ")),
        value(Method::Add, tag("add")),
        value(Method::Sub, tag("sub")),
        value(Method::Neg, tag("neg")),
        value(Method::Eq, tag("eq")),
        value(Method::Gt, tag("gt")),
        value(Method::Lt, tag("lt")),
        value(Method::And, tag("and")),
        value(Method::Or, tag("or")),
        value(Method::Not, tag("not")),
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
