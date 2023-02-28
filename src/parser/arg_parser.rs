use clap::{App, Arg, ArgMatches, SubCommand};
use crate::{password_generator, models};


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
                        .required(true),
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
                let password = sub_matches.value_of("password").unwrap_or("Idk password");
                let site = sub_matches.value_of("site").unwrap_or("`none site`");
                let user = sub_matches.value_of("user").unwrap_or("`none user`");

                println!("password is {}", password);
                println!("user is {}", user);
                println!("site is {}", site);

                models::connection::add_new_password(
                    user.to_string(),
                    site.to_string(),
                    password.to_string()
                )
            }

            else if let Some(sub_matches) = matches.subcommand_matches("genpass") {
                let lenght = sub_matches.value_of("lenght").unwrap_or("10");

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
                        println!("{}", a);
                    }

                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }
        }

        Err(error) => {
            println!("{}", error);
        }
    }
}
