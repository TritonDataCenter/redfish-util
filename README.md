# redfish-util
Utility for accessing the Redfish services on a BMC/Service Processor

## Usage

```
Usage
 target/debug/redfish_util -H HOST -u USERID -p PASSWD -c CMD:[ARG] [-d] [-i] 
 or
 target/debug/redfish_util -e ENTRY -c CMD:[ARG] [-d] [-i]

Options:
    -e, --entry ENTRY   entry from config file
    -H, --host HOST     FQDN or IP address of BMC
    -u, --user USERID   BMC user id
    -p, --passwd PASSWD BMC user password
    -c, --command CMD[:ARG]
                        command
    -d, --debug         Enable debug messages
    -i, --insecure      Toggle insecure mode on
    -h, --help          Display this usage message

To use a config file, specify the path in REDFISH_UTIL_CONF

Information Commands:
---------------------
where CMD can be:
	chassis		Show chassis summary
	system		Show system summary
	version		Show Redfish version

Action Commands:
----------------
where CMD can be:
	nmi		Send NMI to system
	off		Turn system off
	on		Turn system on
	reset		Reset system
	forceoff	Force turn system off
	forceon		Force turn system on
	forcereset	Force reset system
	biossetup	Set next boot to go to BIOS setup mode
	identifyoff	Turn Identify LED off
	identifyon	Turn Identify LED on

optional: where ARG can be the Redfish System ID
defaults to the first system
```

### Configuration File

As a convenience, a JSON onfiguration file can be specified that contains the values for the "host", "user" and "passwd" arguments.   The format of the file is:

```
{
  "entries": [
    {
      "name": "name of entry, pass to -e",
      "host": "FQDN or IP address of BMC",
      "user": "BMC user id",
      "passwd": "BMC user password"
    },
   . . .
     ]
}
```

For example:

```
{
  "entries": [
    {
      "name": "fileserver1-sp",
      "host": "192.168.0.101",
      "user": "ADMIN",
      "passwd": "ADMIN"
    },
    {
      "name": "computenode-sp",
      "host": "192.168.0.102",
      "user": "root",
      "passwd": "secretrootpassword"
    }
  ]
}
```
