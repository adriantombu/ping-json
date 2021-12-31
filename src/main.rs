use regex::Regex;
use serde::Serialize;
use std::process::Command;
use std::str;

#[derive(Serialize, Debug)]
struct PingError {
    error: String,
}

#[derive(Serialize, Debug)]
struct Ping {
    host: String,
    ip: String,
    packets_send: u8,
    packets_received: u8,
    packet_loss: f32,
    min: f32,
    avg: f32,
    max: f32,
    stddev: f32,
}

// TODO: add argument for host
// TODO: add argument for count
// TODO: split per line for smaller regex
// TODO: add a requests array in the output for each request made (determined by count argument)
fn main() {
    let p = ping("otso.404");
    let res = match p {
        Ok(res) => res,
        Err(err) => {
            return println!(
                "{}",
                serde_json::to_string(&PingError { error: err }).unwrap()
            )
        }
    };

    let re = Regex::new(r"PING\s(\S*)\s\(([\d.]+)\).+\n.+\n.+\n.+\n\n.+\n(\d)[\s\w]+,\s(\d)[\s\w]+,\s(\d\.\d)+.*\n[a-z-/=\s]+([\d.]+)/([\d.]+)/([\d.]+)/([\d.]+)").unwrap();
    for cap in re.captures_iter(&res) {
        let res = Ping {
            host: cap[1].to_owned(),
            ip: cap[2].to_owned(),
            packets_send: cap[3].to_owned().parse::<u8>().unwrap(),
            packets_received: cap[4].to_owned().parse::<u8>().unwrap(),
            packet_loss: cap[5].to_owned().parse::<f32>().unwrap(),
            min: cap[6].to_owned().parse::<f32>().unwrap(),
            avg: cap[7].to_owned().parse::<f32>().unwrap(),
            max: cap[8].to_owned().parse::<f32>().unwrap(),
            stddev: cap[9].to_owned().parse::<f32>().unwrap(),
        };

        println!("{}", serde_json::to_string(&res).unwrap())
    }
}

fn ping(host: &str) -> Result<String, String> {
    let output = Command::new("ping")
        .args(["-c 3", host])
        .output()
        .expect("failed to ping website");

    if output.status.code().unwrap() == 0 {
        return Ok(str::from_utf8(&output.stdout).unwrap().to_string());
    }

    Err(str::from_utf8(&output.stderr).unwrap().to_string())
}
