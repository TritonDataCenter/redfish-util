# redfish-util
Utility for accessing the Redfish services on a BMC/Service Processor

## Usage

```
Usage: target/debug/redfish_util -H HOST -u USERID -p PASSWD -c CMD:[ARG] [-d] [-i]

Options:
    -H, --host HOST     FQDN or IP address of BMC
    -u, --user USERID   BMC user id
    -p, --passwd PASSWD BMC user password
    -c, --command CMD[:ARG]
                        command
    -d, --debug         Enable debug messages
    -i, --insecure      Toggle insecure mode on
    -h, --help          Display this usage message

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

optional: where ARG can be the Redfish System ID
defaults to the first system
```

