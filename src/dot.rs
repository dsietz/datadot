use std::result::{Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::thread::{self, JoinHandle};
use std::string::ToString;
use std::any::Any;
use std::str;
use regex::Regex;
use multicast;
use helper::*;
use plugin_mngt::{PluginManager};
use dynamic_reload::{DynamicReload, Lib, Symbol, Search, PlatformName, UpdateState};
use json::JsonValue;


lazy_static! {
    static ref GET: Regex = Regex::new("^get|^GET").unwrap();
    static ref POST: Regex = Regex::new("^post|^POST").unwrap();
    static ref PUT: Regex = Regex::new("^put|^PUT").unwrap();
    static ref DELETE: Regex = Regex::new("^delete|^DELETE").unwrap();
}

pub struct Dot<'a> {
    addr: IpAddr,
    cmd_delimiter: &'a str,
    data: json::JsonValue,
    listener: Option<JoinHandle<()>>,
    plugin_mngr: PluginManager<'a>,
    port: u16,
    speaker: Option<UdpSocket>,
}

impl<'a> Dot<'a>{
    pub fn new(addr_ip: IpAddr, port_number: u16, data_point: &[u8], plugin_dir: Option<&'a str>) -> Dot<'a> {
        let plg_dir = plugin_dir.unwrap_or_else(||"./plugins");
        let mut pm = PluginManager::new(vec![plg_dir], "/tmp/plugins");
        assert!(pm.add_plugin("plugin_helloWorld").is_ok());

        let mut o = json::JsonValue::new_object();
        let mut d = json::JsonValue::new_array();

        for byt in data_point{
            d.push(*byt as u64);
        }

        o["data"] = d;
        
        Dot{
            addr: addr_ip,
            cmd_delimiter: "::",
            data: o,
            listener: None,
            plugin_mngr: pm,
            port: port_number,
            speaker: None,
        }
    }

    pub fn drop_plugins(&mut self) -> std::result::Result<i32, &'static str>{
        println!("dropping plugins...");
        self.plugin_mngr.unload()
    }

    pub fn get_datapoint(&self) -> Vec<u8>{
        let mut arr: Vec<u8> = Vec::new();

        for byt in self.data["data"].members(){
            arr.push(byt.as_u8().unwrap().clone());
        }
        
        arr
    }

    pub fn health(&mut self) -> std::result::Result<i32, &'static str>{
        Ok(1)
    }

    /// This function maps the end user's command (.e.g; SELF::ATTR::name) to the plugin that owns to the 
    /// processing of the command. 
    pub fn process_command(&mut self, message: &[u8]) -> std::result::Result<Box<Any>, &'static str>{
        let msg = str::from_utf8(&message).unwrap();
        let cmds: Vec<&str> = msg.split(CMD_DELIMITER).collect();
        //Ok(Box::new(cmds[0].to_string()))
        
        match 1 {
            1 | _ if GET.is_match(cmds[0]) => {
                match self.speak(cmds[1].as_bytes()) {
                    Ok(v) => Ok(Box::new("want! want! want!".to_string())),
                    Err(e) => Err(e),
                }
            },
            _ => {
                self.plugin_mngr.refresh_plugins();

                let plgs = self.plugin_mngr.get_plugins();

                for plg in self.plugin_mngr.get_plugins() {
                    let pc: Symbol<extern fn(message: &[u8], data: &json::JsonValue) -> json::JsonValue> = unsafe {
                        plg.lib.get(b"process_command\0").unwrap()
                    };

                    self.data = pc(message, &self.data.clone());
                }

                Ok(Box::new("processed...".to_string()))
            }
        }
    }

    pub fn start_listener(&mut self) -> std::result::Result<i32, &'static str>{
        assert!(self.addr.is_multicast());

        let addr = SocketAddr::new(self.addr, self.port);
        self.listener = Some(multicast::multicast_listener(addr));

        Ok(1)
    }

    pub fn stop_listener(&mut self) -> std::result::Result<i32, &'static str>{
        self.speak(b"stop")
    }

    pub fn speak(&mut self, message: &[u8]) -> std::result::Result<i32, &'static str>{
        let addr = SocketAddr::new(self.addr, self.port);
        let sender = multicast::new_sender(&addr);

        match sender {
            Ok(val) => {
                let speaker = Some(val);
                speaker.unwrap().send_to(message, &addr).expect("could not send_to!");
                Ok(1)
            },
            Err(err) => {
                println!("{}",err);
                Err("Warning: Couldn't start the data dot speaker!")
            },
        }
    }
}

/// UNIT TESTING
#[cfg(test)]
mod tests {
    use super::*;
    use std::time;
    use std::time::{Duration, Instant};

    fn wait(){
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        assert!(now.elapsed() >= ten_millis);
    }

    #[test]
    fn test_get_datapoint() {
        let a: IpAddr = Ipv6Addr::new(0xFF03, 0, 0, 0, 0, 0, 0, 0x0123).into();
        let p: u16 = 7645;
        let msg = String::from("datapoint_test");
        let path = Some("./target/debug/examples");
        let dot = Dot::new(a, p, &msg.as_bytes(), path);

        assert_eq!(dot.get_datapoint(), msg.as_bytes());
    }

    #[test]
    fn test_health() {
        let a: IpAddr = Ipv6Addr::new(0xFF03, 0, 0, 0, 0, 0, 0, 0x0123).into();
        let p: u16 = 7645;
        let path = Some("./target/debug/examples");
        let mut dot = Dot::new(a, p, String::from("hello").as_bytes(), path);

        assert!(dot.health().is_ok());
    }

    #[test]
    fn test_communication() {
        let a: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
        let p: u16 = 7645;
        let path = Some("./target/debug/examples");
        let mut dot = Dot::new(a, p, String::from("hello").as_bytes(), path);

        assert!(dot.start_listener().is_ok());
        assert!(dot.process_command(b"echo::first message").is_ok());    

        //wait a few moments
        wait();

        assert!(dot.process_command(b"echo::second message").is_ok()); 
        assert!(dot.stop_listener().is_ok());
    }


    #[test]
    fn test_process_commands() {
        let a: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
        let p: u16 = 7645;
        let path = Some("./target/debug/examples");
        let mut dot = Dot::new(a, p, String::from("hello").as_bytes(), path);
        let ok_cmds = vec!["GET::test"];
 
        for cmd in ok_cmds {
            assert!(dot.process_command(cmd.as_bytes()).is_ok());
        }

        //assert!(dot.process_command(b"blah::test").is_err());  
    }
}
