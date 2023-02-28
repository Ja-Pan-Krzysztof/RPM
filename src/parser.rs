pub mod arg_parser;


#[warn(dead_code)]
pub fn parser(args: Vec<String>) -> u64 {
    let mut lenght: u64 = 12;

    if args.len() > 2 {
        if args[1] == "-len" {
            match args[2].parse::<u64>() {
                Ok(_) => lenght = args[2].trim().parse().unwrap(),
                Err(_) => println!("Wrong value in `len` parameters. Default 10 chars was used."),
            }
        }
    }

    lenght
}