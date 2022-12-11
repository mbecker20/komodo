#![allow(unused)]

use clap::{arg, Arg, Command};

mod helpers;

use helpers::*;

fn cli() -> Command {
    Command::new("monitor_cli")
        .about("\na cli to set up monitor components")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("core")
                .about("tools to set up monitor core")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("config_gen")
                        .about("generate a core config")
                        .arg(
                            arg!(--path <PATH> "sets path of generated config file. default is '~/.monitor/config.toml'")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port core will run on. default is 9000")
                                .required(false)
                        )
                        .arg(
                            arg!(--mongo_uri <URI> "sets the mongo uri to use. default is 'mongodb://mongo'")
                                .required(false)
                        )
                        .arg(
                            arg!(--mongo_db_name <NAME> "sets the db name to use. default is 'monitor'")
                                .required(false)
                        )
                        .arg(
                            arg!(--jwt_valid_for <TIMELENGTH> "sets the length of time jwt stays valid for. default is 1-wk (one week)")
                                .required(false)
                        )
                        .arg(
                            arg!(--slack_url <URL> "sets the slack url to use for slack notifications")
                                .required(false)
                        ),
                )
                .subcommand(Command::new("start_mongo").about("start up a mongo for monitor"))
                .subcommand(Command::new("start").about("start up monitor core")),
        )
        .subcommand(
            Command::new("periphery")
                .about("tools to set up monitor periphery")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("config_gen")
                        .about("generate a periphery config")
                        .arg(
                            arg!(--path <PATH> "sets path of generated config file. default is '~/.monitor/config.toml'")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port periphery will run on. default is 9001")
                                .required(false)
                        )
                        .arg(
                            arg!(--repo_dir <PATH> "sets folder that repos will be cloned into. default is /repos")
                                .required(false)
                        )
                )
                .subcommand(Command::new("start").about("start up monitor periphery")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("core", sub_matches)) => {
            let core_command = sub_matches.subcommand().expect("invalid call, should be 'monitor_cli core <config_gen, start_mongo, start> <flags>'");
            match core_command {
                ("config_gen", sub_matches) => gen_core_config(sub_matches),
                ("start_mongo", sub_matches) => start_mongo(sub_matches),
                ("start", sub_matches) => start_core(sub_matches),
                _ => {
                    println!("invalid call, should be 'monitor_cli core <config_gen, start_mongo, start> <flags>'")
                }
            }
        }
        Some(("periphery", sub_matches)) => {
            let periphery_command = sub_matches.subcommand().expect(
                "invalid call, should be 'monitor_cli periphery <config_gen, start> <flags>'",
            );
            match periphery_command {
                ("config_gen", sub_matches) => gen_periphery_config(sub_matches),
                ("start", sub_matches) => start_periphery(sub_matches),
                _ => {
                    println!("invalid call, should be 'monitor_cli core <config_gen, start_mongo, start> <flags>'")
                }
            }
        }
        _ => unreachable!(),
    }
}
