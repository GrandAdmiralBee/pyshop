use clap::{value_parser, Arg, ArgAction, Command};
use colored::*;

fn cli() -> Command {
    Command::new("hash_finder")
        .about("A hash finder")
        .arg_required_else_help(true)
        .args([
            Arg::new("number-of-zeros")
                .short('N')
                .long("number-of-zeros")
                .value_parser(value_parser!(u32))
                .help("Number of zeros at the end of hashcode")
                .action(ArgAction::Set)
                .num_args(1),
            Arg::new("number-of-codes")
                .short('F')
                .long("number-of-codes")
                .value_parser(value_parser!(u32))
                .help("Number of hashcodes to be printed")
                .action(ArgAction::Set)
                .num_args(1),
        ])
}

fn main() {
    let matches = cli().get_matches();

    let n: Option<&u32> = matches.get_one("number-of-zeros");
    let f: Option<&u32> = matches.get_one("number-of-codes");

    if n.is_none() {
        eprintln!("You must provide -N flag with a value");
        return;
    }

    if f.is_none() {
        eprintln!("You must provide -F flag with a value");
        return;
    }

    let n = *n.unwrap();
    let f = *f.unwrap();

    let regex = format!("([^0]0{{{}}}ы)", n);
    let regex = regex::Regex::new(&regex).unwrap();

    let mut hash_codes: Vec<(u32, String)> = vec![];
    let mut integer = 1;
    loop {
        let hash_code = sha256::digest(integer.to_string());
        let mut val = hash_code.clone();
        integer += 1;

        // Для тогоа, чтобы найти регулярным выражением тольео конец строки
        val.push('ы');

        let capture = regex.captures(&val);
        if capture.is_some() {
            hash_codes.push((integer, hash_code));
            continue;
        }

        if hash_codes.len() >= f as usize {
            break;
        }
    }

    for (num, hash_code) in hash_codes {
        println!("{}, {}", num.to_string().red(), hash_code.green());
    }
}
