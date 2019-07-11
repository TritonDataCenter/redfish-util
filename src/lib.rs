//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2019 Joyent, Inc.
//
extern crate reqwest;
extern crate serde_json;

extern crate serde;

mod redfish;
use redfish::{
    RedfishChassis,
    RedfishCollection,
    RedfishEthernetIntf,
    RedfishManager,
    RedfishPower,
    RedfishProcessor,
    RedfishRootService,
    RedfishStatus,
    RedfishSystem,
};

#[derive(Debug)]
struct SimpleError(String);                                              

impl Error for SimpleError {}                                            
                                                                     
impl fmt::Display for SimpleError {                                      
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RedfishUtilCmd {
    pub cmd: String,
    pub arg: Option<String>,
}

impl RedfishUtilCmd {
    pub fn new(cmd: String, arg: Option<String>) -> RedfishUtilCmd {
        RedfishUtilCmd { cmd, arg }
    }
}

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub insecure: bool,
    pub user: String,
    pub passwd: String,
    pub host: String,
    pub cmd: RedfishUtilCmd,
}

impl Config {
    pub fn new(debug: bool, insecure: bool, user: String, passwd: String,
        host: String, cmd: RedfishUtilCmd) -> Config {
        Config { debug, insecure, user, passwd, host, cmd }
    }
}

fn print_status(status: &RedfishStatus, pad: usize) {
    let pad = " ".repeat(pad);

    if status.state.is_some() {
        println!("{0}{1: <20} {2}", pad, "State:",
            status.state.as_ref().unwrap());
    }
    if status.health.is_some() {
        println!("{0}{1: <20} {2}", pad, "Health:",
            status.health.as_ref().unwrap());
    }
    if status.health_rollup.is_some() {
        println!("{0}{1: <20} {2}", pad, "Health Rollup:",
            status.health_rollup.as_ref().unwrap());
    }
}

fn chassis_get(config: &Config) -> Result<(), Box<dyn Error>> {
    let uri = "/redfish/v1/Chassis";
    let resp = do_get_request(&config, &uri)?;
    let coll: RedfishCollection = serde_json::from_str(&resp)?;

    println!("Number of Chassis: {}", coll.members.len());
    for mmbr in &coll.members {
        let resp = do_get_request(&config, &mmbr.uri)?;
        let mut chassis: RedfishChassis = serde_json::from_str(&resp)?;
        chassis.uri = mmbr.uri.to_string();
        println!("Chassis Details");
        println!("  {0: <20} {1}", "Type:", chassis.chassis_type);
        println!("  {0: <20} {1}", "Manufacturer:", chassis.manufacturer);
        println!("  {0: <20} {1}", "Serial Number:", chassis.serial_num);
        println!("  {0: <20} {1}", "Part Number:", chassis.part_num);
        print_status(&chassis.status, 2);
        power_get(&config, &chassis.power.uri)?;
    }
    Ok(())
}

fn ethernet_get(config: &Config, uri: &str) -> Result<(), Box<dyn Error>> {
    let resp = do_get_request(&config, uri)?;
    let coll: RedfishCollection = serde_json::from_str(&resp)?;

    println!("\n  Ethernet Interfaces");
    for mmbr in &coll.members {
        let resp = do_get_request(&config, &mmbr.uri)?;
        let mut eth: RedfishEthernetIntf = serde_json::from_str(&resp)?;
        eth.uri = mmbr.uri.clone();
        println!("    {0: <20} {1}", "Label:", eth.name);
        if eth.mac_addr.is_some() {
            println!("    {0: <20} {1}", "MAC Address:",
                eth.mac_addr.unwrap());
        }
        if eth.link_state.is_some() {
            println!("    {0: <20} {1}", "Link State:",
                eth.link_state.unwrap());
        }
        if eth.ipv4.is_some() {
            let ipv4addrs = eth.ipv4.unwrap();
            for ipv4 in &ipv4addrs {
                println!("    {0: <20} {1}", "IPv4 Address:",
                    ipv4.address);
                println!("    {0: <20} {1}", "IPv4 Subnet:",
                    ipv4.subnet);
                println!("    {0: <20} {1}", "IPv4 Gateway:",
                    ipv4.gateway);
                println!("    {0: <20} {1}", "IPv4 Source:",
                    ipv4.origin);
            }
        }
        println!("    Status");
        print_status(&eth.status, 6);
        println!();
    }
    Ok(())
}

fn managers_get(config: &Config) -> Result<(), Box<dyn Error>> {
    let uri = "/redfish/v1";
    let resp = do_get_request(&config, &uri)?;
    let rootsvc: RedfishRootService = serde_json::from_str(&resp)?;
    let resp = do_get_request(&config, &rootsvc.mngrs.uri)?;
    let coll: RedfishCollection = serde_json::from_str(&resp)?;
    
    for mmbr in &coll.members {
        let resp = do_get_request(&config, &mmbr.uri)?;
        let mut mngr: RedfishManager = serde_json::from_str(&resp)?;
        mngr.uri = mmbr.uri.clone();
        println!("  {0: <20} {1}", "Type:", mngr.mngr_type);
        if mngr.model.is_some() {
            println!("  {0: <20} {1}", "Model:",
                mngr.model.unwrap());
        }
        if mngr.fw_version.is_some() {
            println!("  {0: <20} {1}", "Firmware Version:",
                mngr.fw_version.unwrap());
        }
        let mut supp_cons = String::new();
        if mngr.cons_graph.is_some() &&
            mngr.cons_graph.unwrap().enabled {
            supp_cons.push_str("KVM ");
        }
        if mngr.cons_serial.is_some() &&
            mngr.cons_serial.unwrap().enabled {
            supp_cons.push_str("Serial ");
        }
        if mngr.cons_shell.is_some() &&
            mngr.cons_shell.unwrap().enabled {
            supp_cons.push_str("CLI");
        }
        println!("  {0: <20} {1}", "Console Types:", supp_cons);
        println!("  Status");
        print_status(&mngr.status, 4);
        if mngr.eth_intfs.is_some() {
            ethernet_get(&config, &mngr.eth_intfs.unwrap().uri)?;
        }
        println!();
    }
    Ok(())
}

fn power_get(config: &Config, uri: &str) -> Result<(), Box<dyn Error>> {
    let resp = do_get_request(&config, uri)?;
    let mut power: RedfishPower = serde_json::from_str(&resp)?;

    println!("\n  Power Supplies");
    for psu in &mut power.power_supplies {
        println!("    {0: <20} {1}", "Label:", psu.name);
        if psu.model.is_some() {
            println!("    {0: <20} {1}", "Model:", psu.model.as_mut().unwrap());
        }
        if psu.serial.is_some() {
            println!("    {0: <20} {1}", "Serial:", psu.serial.as_mut().unwrap());
        }
        print_status(&psu.status, 4);
        println!("");
    }
    Ok(())
}

fn processors_get(config: &Config, uri: &str) -> Result<(), Box<dyn Error>> {
    let resp = do_get_request(&config, uri)?;
    let coll: RedfishCollection = serde_json::from_str(&resp)?;

    println!("\n  Processors");
    for mmbr in &coll.members {
        let resp = do_get_request(&config, &mmbr.uri)?;
        let mut chip: RedfishProcessor = serde_json::from_str(&resp)?;
        chip.uri = mmbr.uri.clone();
        println!();
        println!("    {0: <20} {1}", "Label:", chip.socket);
        println!("    {0: <20} {1}", "Manufacturer:", chip.manufacturer);
        println!("    {0: <20} {1}", "Brand:", chip.brand);
        if chip.id.family.is_some() {
            println!("    {0: <20} {1}", "Family:", chip.id.family.unwrap());
        }
        if chip.id.model.is_some() {
            println!("    {0: <20} {1}", "Model:", chip.id.model.unwrap());
        }
        if chip.id.stepping.is_some() {
            println!("    {0: <20} {1}", "Stepping:",
                chip.id.stepping.unwrap());
        }
        if chip.id.ucode_version.is_some() {
            println!("    {0: <20} {1}", "Ucode Version:",
                chip.id.ucode_version.unwrap());
        }
        println!("    {0: <20} {1} MHz", "Speed:", chip.speed_mhz);
        println!("    {0: <20} {1}", "Total Cores:", chip.ncores);
        println!("    {0: <20} {1}", "Total Threads:", chip.nthreads);
        println!("    Status");
        print_status(&chip.status, 6);
    }
    Ok(())
}

fn system_get(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Managers");
    managers_get(&config)?;

    let uri = "/redfish/v1/Systems";
    let resp = do_get_request(&config, &uri)?;
    let coll: RedfishCollection = serde_json::from_str(&resp)?;

    for mmbr in &coll.members {
        let resp = do_get_request(&config, &mmbr.uri)?;
        let mut system: RedfishSystem = serde_json::from_str(&resp)?;
        system.uri = mmbr.uri.to_string();
        println!("System Details");
        println!("  {0: <20} {1}", "Type:", system.sys_type);
        println!("  {0: <20} {1}", "Manufacturer:", system.manufacturer);
        println!("  {0: <20} {1}", "Model:", system.model);
        println!("  {0: <20} {1}", "Serial Number:", system.serial_num);
        println!("  {0: <20} {1}", "Part Number:", system.part_num);
        if system.sku.is_some() {
            println!("  {0: <20} {1}", "SKU:", system.sku.unwrap());
        };
        if system.uuid.is_some() {
            println!("  {0: <20} {1}", "UUID:", system.uuid.unwrap());
        };
        println!("  {0: <20} {1}", "BIOS Version:", system.bios_vers);
        if system.pwr_state.is_some() {
            println!("  {0: <20} {1}", "Power Status:",
                system.pwr_state.unwrap());
        }
        if system.locate_led.is_some() {
            println!("  {0: <20} {1}", "Locate LED:",
                system.locate_led.unwrap());
        }
        println!("  Status");
        print_status(&system.memory.status, 4);
        processors_get(&config, &system.chips.uri)?;
        println!("\n  Memory");
        println!("    {0: <20} {1} GiB", "Total RAM:",
            system.memory.total_memory);
        if system.eth_intfs.is_some() {
            ethernet_get(&config, &system.eth_intfs.unwrap().uri)?;
        }
    }
    Ok(())
}

fn version_get(config: &Config) -> Result<(), Box<dyn Error>> {
    let uri = "/redfish/v1";
    let resp = do_get_request(&config, &uri)?;
    let rootsvc: RedfishRootService = serde_json::from_str(&resp)?;

    println!("Redfish version: {}", rootsvc.version);

    Ok(())
}

fn do_get_request(config: &Config, uri: &str)
    -> Result<String, Box<dyn Error>> {
    let req_url = format!("https://{}{}", config.host, uri);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure)
        .build()?;

    if config.debug {
        eprintln!("Sending Request: {}", req_url);
    }
    let mut response = client.get(&req_url)
        .basic_auth(&config.user, Some(&config.passwd))
        .send()
        .expect("Failed to send request");
    
    if response.status().is_success() {
        let resp_txt = response.text().unwrap();
        if config.debug {
            eprintln!("Response:\n{}\n", &resp_txt);
        }
        Ok(resp_txt)
    } else {
        Err(Box::new(SimpleError(
            format!("Request Failed! - Status Code: {}", response.status()))))
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    match config.cmd.cmd.as_ref() {
        "chassis" => { chassis_get(&config)? }
        "system" => { system_get(&config)? }
        "version" => { version_get(&config)? }
        _ => { panic!("unexpected command"); }
    };
    
    Ok(())
}
