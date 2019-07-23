//
// Copyright 2019 Joyent, Inc.
//
extern crate serde;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct RedfishRootService {
    #[serde(rename = "RedfishVersion")]
    pub version: String,
    #[serde(rename = "Managers")]
    pub mngrs: RedfishMember,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishMember {
    #[serde(rename = "@odata.id")]
    pub uri: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishCollection {
    #[serde(rename = "Members")]
    pub members: Vec<RedfishMember>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishActionReset {
    pub target: String,
    #[serde(rename = "ResetType@Redfish.AllowableValues")]
    pub reset_type: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishConsole {
    #[serde(rename = "ServiceEnabled")]
    pub enabled: bool,
    #[serde(rename = "MaxConcurrentSessions")]
    pub max_sessions: i32,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishChassis {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "ChassisType")]
    pub chassis_type: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "SerialNumber")]
    pub serial_num: String,
    #[serde(rename = "PartNumber")]
    pub part_num: String,
    #[serde(rename = "Power")]
    pub power: RedfishMember,
    #[serde(rename = "Thermal")]
    pub thermal: RedfishMember,
    #[serde(rename = "Status")]
    pub status: Option<RedfishStatus>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishEthernetIntf {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "UefiDevicePath")]
    pub uefi_devpath: Option<String>,
    #[serde(rename = "MACAddress")]
    pub mac_addr: Option<String>,
    #[serde(rename = "LinkStatus")]
    pub link_state: Option<String>,
    #[serde(rename = "IPv4Addresses")]
    pub ipv4: Option<Vec<RedfishIpv4Address>>,
    #[serde(rename = "SpeedMbps")]
    pub link_speed: Option<i32>,
    #[serde(rename = "MTUSize")]
    pub link_mtu: Option<i32>,
    #[serde(rename = "InterfaceEnabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "AutoNeg")]
    pub auto_neg: Option<bool>,
    #[serde(rename = "FullDuplex")]
    pub full_duplex: Option<bool>,
    #[serde(rename = "FQDN")]
    pub fqdn: Option<String>,
    #[serde(rename = "HostName")]
    pub hostname: Option<String>,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishFan {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "FanName")]
    pub name: String,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishIpv4Address {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "SubnetMask")]
    pub subnet: String,
    #[serde(rename = "AddressOrigin")]
    pub origin: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishManager {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "ManagerType")]
    pub mngr_type: String,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "FirmwareVersion")]
    pub fw_version: Option<String>,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
    #[serde(rename = "EthernetInterfaces")]
    pub eth_intfs: Option<RedfishMember>,
    #[serde(rename = "GraphicalConsole")]
    pub cons_graph: Option<RedfishConsole>,
    #[serde(rename = "SerialConsole")]
    pub cons_serial: Option<RedfishConsole>,
    #[serde(rename = "CommandShell")]
    pub cons_shell: Option<RedfishConsole>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishMemorySummary {
    #[serde(rename = "TotalSystemMemoryGiB")]
    pub total_memory: i32,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishPower {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "PowerSupplies")]
    pub power_supplies: Vec<RedfishPowerSupply>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishPowerSupply {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial: Option<String>,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishProcessorId {
    #[serde(rename = "EffectiveFamily")]
    pub family: Option<String>,
    #[serde(rename = "EffectiveModel")]
    pub model: Option<String>,
    #[serde(rename = "Step")]
    pub stepping: Option<String>,
    #[serde(rename = "MicrocodeInfo")]
    pub ucode_version: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishProcessor {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "Socket")]
    pub socket: String,
    #[serde(rename = "Model")]
    pub brand: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "MaxSpeedMHz")]
    pub speed_mhz: i32,
    #[serde(rename = "TotalCores")]
    pub ncores: i32,
    #[serde(rename = "TotalThreads")]
    pub nthreads: i32,
    #[serde(rename = "ProcessorArchitecture")]
    pub arch: String,
    #[serde(rename = "InstructionSet")]
    pub isa: String,
    #[serde(rename = "ProcessorId")]
    pub id: RedfishProcessorId,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishProcessorSummary {
    #[serde(rename = "Count")]
    pub ncpus: i32,
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Status")]
    pub status: RedfishStatus,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishStatus {
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<String>,
    #[serde(rename = "HealthRollup")]
    pub health_rollup: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishSystem {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "SerialNumber")]
    pub serial_num: String,
    #[serde(rename = "PartNumber")]
    pub part_num: String,
    #[serde(rename = "SystemType")]
    pub sys_type: String,
    #[serde(rename = "BiosVersion")]
    pub bios_vers: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "SKU")]
    pub sku: Option<String>,
    #[serde(rename = "UUID")]
    pub uuid: Option<String>,
    #[serde(rename = "PowerState")]
    pub pwr_state: Option<String>,
    #[serde(rename = "IndicatorLED")]
    pub locate_led: Option<String>,
    #[serde(rename = "ProcessorSummary")]
    pub chip_summary: RedfishProcessorSummary,
    #[serde(rename = "Processors")]
    pub chips: RedfishMember,
    #[serde(rename = "MemorySummary")]
    pub memory: RedfishMemorySummary,
    #[serde(rename = "EthernetInterfaces")]
    pub eth_intfs: Option<RedfishMember>,
    #[serde(rename = "Actions")]
    pub actions: RedfishSystemActions,
    #[serde(rename = "Boot")]
    pub boot: Option<RedfishSystemBoot>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishSystemActions {
    #[serde(rename = "#ComputerSystem.Reset")]
    pub reset: Option<RedfishActionReset>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishSystemBoot {
    #[serde(rename = "BootSourceOverrideEnabled")]
    pub override_state: Option<String>,
    #[serde(rename = "BootSourceOverrideTarget")]
    pub override_target: Option<String>,
    #[serde(rename = "BootSourceOverrideTarget@Redfish.AllowableValues")]
    pub override_alltargets: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RedfishThermal {
    #[serde(skip)]
    pub uri: String,
    #[serde(rename = "Fans")]
    pub fans: Vec<RedfishFan>,
}
