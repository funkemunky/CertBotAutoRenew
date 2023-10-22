use std::env;
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::Duration;

// 6 hours
const DEFAULT_RUN_TIME: Duration = Duration::from_secs(21600);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args.len() > 2 {
            let ttl = args[2].parse::<u64>().unwrap();
            let runtime = Duration::from_secs(ttl);
            process_renewal(&args[1], runtime);
        } else {
            process_renewal(&args[1], DEFAULT_RUN_TIME);
        }
    } else {
        eprintln!("Use the following arguments: <domain> [ttl_seconds]");
    }
}

// Command: certbot certonly --standalone -d <domain>

const CERTBOT_COMMAND: &str = "certbot  certonly --standalone -d <domain>";


fn process_renewal(domain: &str, runtime: Duration) {
    loop {
        let command_formatted = CERTBOT_COMMAND.replace("<domain>", domain);

        let output = Command::new("bash")
            .arg("-c")
            .arg(command_formatted)
            .output();

        match output {
            Ok(output) => {
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));

                println!("Sleeping...");
                sleep(runtime);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                sleep(Duration::from_secs(5));
            }
        }
    }
}