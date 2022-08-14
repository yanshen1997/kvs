// Note: this requires the `cargo` feature
use kvs::KvStore;
use std::process::exit;

fn main() {
    let cmd_matches = clap::Command::new("kvs")
        .bin_name("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(
            clap::command!("get")
                .about("Get the string value of a given string key")
                .arg(
                    clap::arg!(<KEY>)
                        .required(true)
                        .help("A string key")
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            clap::command!("set")
                .about("Set the value of a string key to a string")
                .arg(
                    clap::arg!(<KEY>)
                        .required(true)
                        .help("A string key")
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    clap::arg!(<VALUE>)
                        .required(true)
                        .help("the value of string key")
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            clap::command!("rm").about("Remove a given key").arg(
                clap::arg!(<KEY>)
                    .required(true)
                    .help("A string key")
                    .value_parser(clap::value_parser!(String)),
            ),
        )
        .get_matches();

    let mut kv_store = KvStore::new();

    // 通过测试
    // match cmd_matches.subcommand() {
    //     Some(("set", _matches)) => {
    //         eprintln!("unimplemented");
    //         exit(1);
    //     }
    //     Some(("get", _matches)) => {
    //         eprintln!("unimplemented");
    //         exit(1);
    //     }
    //     Some(("rm", _matches)) => {
    //         eprintln!("unimplemented");
    //         exit(1);
    //     }
    //     _ => unreachable!(),
    // }
    match cmd_matches.subcommand() {
        Some(("get", sub_matches)) => {
            if let Some(key) = sub_matches.get_one::<String>("KEY").cloned() {
                if let Some(val) = kv_store.get(key) {
                    println!("{}",val)
                }else {
                    println!("none")
                }
            }
        }
        Some(("set", sub_matches)) => {
            if let (Some(key), Some(value)) = (
                sub_matches.get_one::<String>("KEY").cloned(),
                sub_matches.get_one::<String>("VALUE").cloned(),
            ) {
                kv_store.set(key, value);
                println!("ok.")
            }
        }
        Some(("rm", sub_matches)) => {
            if let Some(key) = sub_matches.get_one::<String>("KEY").cloned() {
                kv_store.remove(key);
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
