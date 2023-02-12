use std::error::Error;

use clap::Parser;
use nom::{
    bytes::{complete::take_until, complete::take_until1},
    multi::many0,
    IResult,
};

#[derive(Parser, Debug)]
struct Args {
    recipes: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum DependencyKind {
    TARGET,
    FILE,
}

#[derive(Debug, PartialEq)]
pub struct CakeDependency {
    pub kind: DependencyKind,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CakeRule {
    pub target: String,
    pub dependencies: Vec<CakeDependency>,
    pub outputs: Vec<String>,
    pub body: String,
}

#[derive(Debug)]
pub struct Cakefile {
    pub rules: Vec<CakeRule>,
}

fn cakefile(input: &str) -> IResult<&str, Cakefile> {
    let (input, rules) = many0(rule)(input)?;
    Ok((input, Cakefile { rules }))
}

fn rule(input: &str) -> IResult<&str, CakeRule> {
    let (input, target) = take_until1(":")(input)?;
    let target = target.to_string();
    let (input, dependencies) = many0(dependency)(input)?;
    let (input, outputs) = many0(output)(input)?;
    let (input, body) = take_until("\n\n")(input)?;
    let body = body.to_string();

    Ok((
        input,
        CakeRule {
            target,
            body,
            dependencies,
            outputs,
        },
    ))
}

fn dependency(input: &str) -> IResult<&str, CakeDependency> {
    Ok((input, CakeDependency { kind, name }))
}

fn output(input: &str) -> IResult<&str, String> {
    Ok((input, output))
}
