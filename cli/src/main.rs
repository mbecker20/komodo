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
                        .about("generate a core config file")
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
                .subcommand(
                    Command::new("start_mongo")
                        .about("start up a local mongo container for monitor")
                        .arg(
                            arg!(--name <NAME> "specify the name of the mongo container. default is monitor-mongo")
                                .required(false)
                        )
                        .arg(
                            arg!(--username <USERNAME> "specify the admin username for mongo. default is mongo with no auth")
                                .required(false)
                        )
                        .arg(
                            arg!(--password <PASSWORD> "specify the admin password for mongo. default is mongo with no auth")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port mongo will run on. default is 27017")
                                .required(false)
                        )
                        .arg(
                            arg!(--mount <PATH> "sets the path the mongo data is mounted into. default is ~/.monitor/db")
                                .required(false)
                        )
                        .arg(
                            arg!(--network <NETWORK> "sets docker network of mongo container. default is bridge")
                                .required(false)
                        )
                )
                .subcommand(
                    Command::new("start")
                        .about("start up monitor core")
                        .arg(
                            arg!(--config_path <PATH> "specify the file path to use for config. default is ~/.monitor/config.toml")
                                .required(false)
                        )
                ),
        )
        .subcommand(
            Command::new("periphery")
                .about("tools to set up monitor periphery")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("config_gen")
                        .about("generate a periphery config file")
                        .arg(
                            arg!(--path <PATH> "sets path of generated config file. default is '~/.monitor/config.toml'")
                                .required(false)
                        )
                        .arg(
                            arg!(--port <PORT> "sets port periphery will run on. default is 9001")
                                .required(false)
                        )
                )
                .subcommand(
                    Command::new("start")
                        .about("start up monitor periphery")
                        .arg(
                            arg!(--config_path <PATH> "specify the file path to use for config. default is ~/.monitor/config.toml")
                                .required(false)
                        )
                        .arg(
                            arg!(--repo_dir <PATH> "specify the folder on system to use as cloning destination. default is ~/.monitor/repos")
                                .required(false)
                        )
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("core", sub_matches)) => {
            let core_command = sub_matches.subcommand().expect("\n❌ invalid call, should be 'monitor_cli core <config_gen, start_mongo, start> <flags>' ❌\n");
            match core_command {
                ("config_gen", sub_matches) => gen_core_config(sub_matches),
                ("start_mongo", sub_matches) => start_mongo(sub_matches),
                ("start", sub_matches) => start_core(sub_matches),
                _ => {
                    println!("\n❌ invalid call, should be 'monitor_cli core <config_gen, start_mongo, start> <flags>' ❌\n")
                }
            }
        }
        Some(("periphery", sub_matches)) => {
            let periphery_command = sub_matches.subcommand().expect(
                "\n❌ invalid call, should be 'monitor_cli periphery <config_gen, start> <flags>' ❌\n",
            );
            match periphery_command {
                ("config_gen", sub_matches) => gen_periphery_config(sub_matches),
                ("start", sub_matches) => start_periphery(sub_matches),
                _ => {
                    println!("\n❌ invalid call, should be 'monitor_cli periphery <config_gen, start> <flags>' ❌\n")
                }
            }
        }
        _ => unreachable!(),
    }
}
