use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not to many arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        } 
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want \r\n -h or -help to how this help message")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let flag = args[1].clone();
    let threads = args[2].clone();
    let ipaddr = args[3].clone();

    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
}

