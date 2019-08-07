//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2019 Joyent, Inc.
//
extern crate getopts;
use getopts::Options;

extern crate serde;
use serde::Deserialize;

extern crate redfish_util;

use std::env;
use std::error::Error;
use std::fs;
use std::process;


#[derive(Debug, Default, Deserialize)]
pub struct ConfigFileEntry {
    pub name: String,
    pub host: String,
    pub user: String,
    pub passwd: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ConfigFile {
    #[serde(skip)]
    pub filename: String,
    pub entries: Vec<ConfigFileEntry>,
}

fn usage(progname: &str, opts: &Options) {
    let msg = format!("Usage\n \
        {} -H HOST -u USERID -p PASSWD -c CMD:[ARG] [-d] [-i] \
        \nor\n \
        {} -e ENTRY -c CMD:[ARG] [-d] [-i]",
        progname, progname
    );
    print!("{}", opts.usage(&msg));
    println!("\nTo use a config file, specify the path in REDFISH_UTIL_CONF");
    println!("\nInformation Commands:");
    println!("---------------------");
    println!("where CMD can be:");
    println!("\tchassis\t\tShow chassis summary");
    println!("\tsystem\t\tShow system summary");
    println!("\tversion\t\tShow Redfish version");
    println!("\nAction Commands:");
    println!("----------------");
    println!("where CMD can be:");
    println!("\tnmi\t\tSend NMI to system");
    println!("\toff\t\tTurn system off");
    println!("\ton\t\tTurn system on");
    println!("\treset\t\tReset system");
    println!("\tforceoff\tForce turn system off");
    println!("\tforceon\t\tForce turn system on");
    println!("\tforcereset\tForce reset system");
    println!("\tbiossetup\tSet next boot to go to BIOS setup mode");
    println!("\tidentifyoff\tTurn Identify LED off");
    println!("\tidentifyon\tTurn Identify LED on");
    println!("\noptional: where ARG can be the Redfish System ID");
    println!("defaults to the first system");
}

fn read_config_file(config_path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    let config_contents = fs::read_to_string(&config_path)?;
    let cfgfile: ConfigFile = serde_json::from_str(&config_contents)?;

    Ok(cfgfile)
}

fn main()  -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("e", "entry", "entry from config file", "ENTRY");
    opts.optopt("H", "host", "FQDN or IP address of BMC", "HOST");
    opts.optopt("u", "user", "BMC user id", "USERID");
    opts.optopt("p", "passwd", "BMC user password", "PASSWD");
    opts.optopt("c", "command", "command", "CMD[:ARG]");
    opts.optflag("d", "debug", "Enable debug messages");
    opts.optflag("i", "insecure", "Toggle insecure mode on");
    opts.optflag("h", "help", "Display this usage message");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        usage(&progname, &opts);
        process::exit(2);
    }

    let debug = matches.opt_present("d");
    let insecure = matches.opt_present("i");
    let cmd = match matches.opt_str("c") {
        Some(c) => {
            let v: Vec<&str> = c.split(':').collect();
            if v.len() == 1 {
                redfish_util::RedfishUtilCmd::new(v[0].to_string(), None)
            } else if v.len() == 2 {
                redfish_util::RedfishUtilCmd::new(v[0].to_string(), Some(v[1].to_string()))
            } else {
                eprintln!("invalid cmd argument");
                usage(&progname, &opts);
                process::exit(2);
            }
        }
        None => {
            eprintln!("-c argument is required");
            usage(&progname, &opts);
            process::exit(2);
        }
    };

    let cfg_path = env::var("REDFISH_UTIL_CONF").ok();
    let config = match matches.opt_str("e") {
        Some(ename) => {
            if cfg_path.is_none() {
                eprintln!("REDFISH_UTIL_CONF is not set!");
                process::exit(1);
            }
            let cfg_file = read_config_file(&cfg_path.unwrap())?;
    
            let mut i = 0;
            for entry in cfg_file.entries.iter() {
                if entry.name == ename {
                    break;
                }
                i += 1;
            }
            if i == cfg_file.entries.len() {
                eprintln!("Couldn't find entry named: {}", ename);
                process::exit(1);
            }
            redfish_util::Config::new(debug, insecure,
                cfg_file.entries[i].user.clone(),
                cfg_file.entries[i].passwd.clone(),
                cfg_file.entries[i].host.clone(), cmd)
        }
        None => {
            let host = match matches.opt_str("H") {
                Some(h) => h,
                None => {
                    eprintln!("-h argument is required");
                    usage(&progname, &opts);
                    process::exit(2);
                }
            };
            let user = match matches.opt_str("u") {
                Some(u) => u,
                None => {
                    eprintln!("-u argument is required");
                    usage(&progname, &opts);
                    process::exit(2);
                }
            };
            let passwd = match matches.opt_str("p") {
                Some(p) => p,
                None => {
                    eprintln!("-u argument is required");
                    usage(&progname, &opts);
                    process::exit(2);
                }
            };
            redfish_util::Config::new(debug, insecure, user, passwd, host, cmd)
        }
    };

    match redfish_util::run(&config) {
        Ok(_r) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e.to_string());
            process::exit(1);
        }
    }
}
