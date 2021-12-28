extern crate ini;
extern crate syslog_loose;

use self::ini::Ini;
use std::net::UdpSocket;
use syslog_loose::SyslogSeverity;

const PORT_NAME_PREFIX: &str = "GigabitEthernet";

pub fn severity_color(severity: SyslogSeverity) -> &'static str {
    match severity {
        SyslogSeverity::SEV_EMERG => "red",
        SyslogSeverity::SEV_ALERT => "red",
        SyslogSeverity::SEV_CRIT => "red",
        SyslogSeverity::SEV_ERR => "red",
        SyslogSeverity::SEV_WARNING => "yellow",
        SyslogSeverity::SEV_NOTICE => "blue",
        SyslogSeverity::SEV_INFO => "blue",
        SyslogSeverity::SEV_DEBUG => "blue",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Ini::load_from_file("/etc/cisco-logger.conf").expect("Cannot open config file");
    let bind_port = conf
        .section(Some("listen"))
        .and_then(|x| x.get("address").cloned())
        .unwrap_or("0.0.0.0:514".to_string());
    let port_section = conf.section(Some("ports")).unwrap();

    let socket = UdpSocket::bind(&bind_port)?;
    println!(
        "{}",
        paris::formatter::colorize_string(format!(
            "📥 <b>cisco-logger</> started, listening for events on <u>{}</> (udp)...",
            bind_port
        ))
    );

    let mut buf = [0u8; 2048];
    loop {
        let (data_read, _) = socket.recv_from(&mut buf)?;
        let line = std::str::from_utf8(&buf[0..data_read])?;
        let msg = syslog_loose::parse_message(&line);

        let appname: String = msg
            .appname
            .unwrap_or("%")
            .to_string()
            .chars()
            .skip(1)
            .collect();
        let mut message: String = msg.msg.to_string();
        let severity = msg.severity.unwrap_or(SyslogSeverity::SEV_NOTICE);

        //check if we have a message for specified `gigabit port`
        if let Some(idx) = message.find(PORT_NAME_PREFIX) {
            let mut d = message
                .chars()
                .skip(idx)
                .skip(PORT_NAME_PREFIX.chars().count());
            let mut no: String = "".to_string();
            for _ in 0..=1 {
                let ch = d.next().unwrap_or(' ');
                if ch != ' ' {
                    no.push(ch);
                }
            }
            //search for port description and try to replace it in a message
            match port_section.get(&no) {
                Some(desc) => {
                    message = message.replace(
                        &format!("{}{}", PORT_NAME_PREFIX, no),
                        &format!("<b>{}</> (<black>port#</> <green>{}</>)", desc, no),
                    );
                }
                _ => (),
            }
        }

        //print final message line
        println!(
            "{}",
            paris::formatter::colorize_string(format!(
                "<{}>[{}]</> <black><i>{}:</> {}",
                severity_color(severity),
                severity.as_str(),
                appname,
                message
            ))
        );
    }
}
