# 📥 cisco-logger

## Genesis
My cisco switch can be configured to send a syslog messages to a syslog server. This is a nice feature because a server can collect this information in its own log. Commonly it is a regular [syslogd](https://en.wikipedia.org/wiki/Syslog)/[rsyslogd](https://en.wikipedia.org/wiki/Rsyslog) or some compatible server, but at my linux box I am using a systemd journal for logging instead.

I was using a "poor man's syslog server" described [here](https://unix.stackexchange.com/a/218791), but despite its simplicity, after some time I can see some limitations of this approach and I decided to write my own tool to suit my needs...

## About
It is a small tool which is parsing an RFC 3164 [syslog](https://en.wikipedia.org/wiki/Syslog) messages produced by cisco hardware. I've tested it on `Cisco Sx220 Series` only, but with some luck it should work on more cisco hardware (as long as the syslog message format stays the same, and chances are good - as this is the RFC standard).

## Features
One of the main feature is formatting syslog messages to have a fancy (colored) and user-friendly look. Main problem for me was that I've got for instance this message produced by original switch:<br>
``<189> Dec 24 2021 21:18:13 192.168.1.1 %Port-5: GigabitEthernet11 link up``<br>
This unfortunatelly besides port number is not telling me __what__ exactly is connected to this #11 port.<br>
Even when the description of the port is properly set in the cisco itself, it is not directly exposing this information in the log.<br>
I decided to create a simple config file where I am assigning port numbers to user-friendly descriptions with information what is connected to specified ports.<br>
As the result the tool is able to log the above information as you can see below:<br>
![Sample output](https://github.com/manio/cisco-logger/blob/master/images/cisco-logger.png)

## Limitations
Because I have only one cisco switch in my network I don't need to inform about its IP address with every message, so I just strip this out. The same applies to facility which seems to be always set to `LOG_LOCAL7`.<br>
Messages also contains the timestamp. The problem with this however is the precision. It is with second resolution only. As the result I decided to also ignore this information, especially because I am running this tool from the systemd unit and every message is timestamped internally by systemd, so printing this information is totally useless. As a bonus systemd is allowing to show the message with better precision (look below for example).

## Config file
The binary is searching for the following config file:<br>
`/etc/cisco-logger.conf`<br>
A sample file may have the following contents:<br>
```
[listen]
address = 0.0.0.0:514

[ports]
3 = "Wi-Fi Access Point 📡"
4 = "Raspberry Pi"
5 = "Laptop 💻"
8 = "Computer 🖥️"
14 = "ethlcd 📟"
```
Port description is an optional thing. If you don't provide port descriptions (or provide it only for some of them) it will just show the original port names where it cannot match a description.

## systemd integration
A sample service file for systemd is here:<br>
[systemd/cisco-logger.service](https://github.com/manio/cisco-logger/blob/master/systemd/cisco-logger.service)
You need to adjust it for your needs (eg. check the binary path).<br>
After placing the unit file in correct location and reloading systemd, the unit can be started as usual:<br>
`systemctl start cisco-logger.service`<br>
Viewing the log (with colors) and precise timestamp is possible using the following command:<br>
`journalctl -f -a -o short-precise -u cisco-logger`

