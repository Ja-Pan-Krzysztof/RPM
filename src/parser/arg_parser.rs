use clap::{App, Arg, ArgMatches, SubCommand};
use crate::{password_generator, models};


fn get_password(lenght: &str) -> String {
    match lenght.parse::<u64>() {
        Ok(len) => {
            let config = password_generator::config::Config {
                enable_digit: true,
                enable_lower: true,
                enable_upper: true,
                enable_special: true,
                pass_lenght: len,
            };

            let a = password_generator::gen_pass::gen_pass(config.pass_lenght, &mut config.new());

            return a;
        }

        Err(error) => {
            panic!("{}", error);
        }
    }
}


fn parse_args() -> clap::Result<ArgMatches> {
    let matches = App::new("JPK")
        .version("1.0")
        .author("Ja-Pan-Krzysztof")
        .about("Generate password or storage your password in safe place")
        .subcommand(
            SubCommand::with_name("genpass")
                .about("Generate password")
                .arg(
                    Arg::new("lenght")
                            .short('l')
                            .long("lenght")
                            .value_name("INTEGER")
                            .help("Set lenght passworg that will be generated (Default 10 chars)")
                            .takes_value(true)
                            .required(false)
                )
        )
        .subcommand(
            SubCommand::with_name("savepass")
                .about("Storage the password")
                .arg(
                    Arg::new("user")
                        .short('u')
                        .long("user")
                        .value_name("STRING")
                        .help("Combine user with password")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("site")
                        .short('s')
                        .long("site")
                        .value_name("STRING")
                        .help("Combine site with password")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("password")
                        .short('p')
                        .long("password")
                        .value_name("STRING")
                        .help("The password that will be wrote in safe place")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::new("genpass")
                        .short('g')
                        .long("genpass")
                        .help("Generate password and save in db with other parameters")
                        .takes_value(false)
                        .required(false)
                )
                .arg(
                    Arg::new("lenght")
                            .short('l')
                            .long("lenght")
                            .value_name("INTEGER")
                            .help("Set lenght passworg that will be generated (Default 10 chars)")
                            .takes_value(true)
                            .required(false)
                )
        )
        .get_matches_safe();

    matches
}

pub fn options() {
    let matches = parse_args();

    match matches {
        Ok(matches) => {
            if let Some(sub_matches) = matches.subcommand_matches("savepass") {
                let site = sub_matches.value_of("site").unwrap_or("");
                let user = sub_matches.value_of("user").unwrap_or("");
                let mut password = String::new();

                if sub_matches.is_present("genpass") {
                    let len = sub_matches.value_of("lenght").unwrap_or("10");

                    password = get_password(len);
                }
                else {
                    password = sub_matches.value_of("password")
                        .unwrap_or("")
                        .to_string();

                    if password == "" {
                        return eprintln!("[Not OK] The password must not be `None`");
                    }
                }

                models::connection::add_new_password(
                    user.to_string(),
                    site.to_string(),
                    password.to_string()
                )
            }

            else if let Some(sub_matches) = matches.subcommand_matches("genpass") {
                let lenght = sub_matches.value_of("lenght").unwrap_or("10");

                println!("[OK] {}", get_password(lenght));
            }
        }

        Err(error) => {
            println!("{}", error);
        }
    }
}
