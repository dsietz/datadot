extern crate dotlib;
extern crate clap;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::BufRead;
use dotlib::dot::{Dot};
use clap::{App, Arg};

pub fn main() {
    let args = App::new("DataDot")
                        // Regular App configuration goes here...
                        .arg(Arg::with_name("data")
                                    .help("The data that the dot represents") 
                                    .takes_value(true)             
                                    .short("d")
                                    .long("data")
                                    .multiple(false)
                                    .required(true)
                        )
                        .arg(Arg::with_name("bind")
                                    .help("The multicast address to bind to (e.g.: 224.0.1.255")
                                    .takes_value(true)            
                                    .short("b")                    
                                    .long("bind")                 
                                    .multiple(false)                
                                    .required(true)                
                        )
                        .arg(Arg::with_name("plugin_dir")
                                    .help("[Optional] The path to the directory where the plugins are located.")
                                    .takes_value(true)            
                                    .short("x")                    
                                    .long("plg_dir")                 
                                    .multiple(false)                
                                    .required(false)                
                        )
                        .arg(Arg::with_name("port")
                                    .help("[Optional] The port number to use for multicasting, (default 7645).")
                                    .takes_value(true)            
                                    .short("p")                    
                                    .long("port")                 
                                    .multiple(false)                
                                    .required(false)                
                        )
                        .get_matches();

    let datapoint = args.value_of("data").unwrap().to_owned();
    let bind = args.value_of("bind").unwrap().to_owned();
    let bnd: Vec<u8> = bind.split(".").map(|b|{b.parse::<u8>().unwrap()}).collect();
    let a: IpAddr = Ipv4Addr::new(bnd[0], bnd[1], bnd[2], bnd[3]).into();
    let plg_dir = args.value_of("plugin_dir").to_owned();
    let p = args.value_of("port").unwrap_or_else(||"7645").to_owned().parse::<u16>().unwrap();
    let mut dot = Dot::new(a, p, datapoint.as_bytes(), plg_dir);

    assert!(dot.start_listener().is_ok());

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            assert!(dot.stop_listener().is_ok());
            assert!(dot.drop_plugins().is_ok());
            break;
        } else {
            //println!("processed: {}", dot.process_command(line.as_bytes()).is_ok());
            
            match dot.process_command(line.as_bytes()){
                Ok(v) => {
                    println!("{:?}",*v.downcast_ref::<String>().unwrap());
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            
        };
    }
    
    println!("Bye!");
}
