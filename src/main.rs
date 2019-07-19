//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2019 Joyent, Inc.
//
extern crate getopts;
use getopts::Options;

use std::env;
use std::process;

extern crate redfish_util;

fn usage(progname: &str, opts: &Options) {
    let msg = format!(
        "Usage: {} -H HOST -u USERID -p PASSWD -c CMD:[ARG] [-d] [-i]",
        progname
    );
    print!("{}", opts.usage(&msg));
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
    println!("\noptional: where ARG can be the Redfish System ID");
    println!("defaults to the first system");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].clone();

    let mut opts = Options::new();
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
    let debug = matches.opt_present("d");
    let insecure = matches.opt_present("i");

    if matches.opt_present("h") {
        usage(&progname, &opts);
        process::exit(2);
    }

    // XXX Add code to validate command
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

    let config = redfish_util::Config::new(debug, insecure, user, passwd, host, cmd);

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
