#[macro_use]
extern crate dotlib;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate json;

use std::str;
use regex::Regex;
use std::any::Any;
use json::JsonValue;
use dotlib::helper::CMD_DELIMITER;

lazy_static! {
    static ref HELLO: Regex = Regex::new("^hello|^HELLO").unwrap();
}

#[no_mangle]
pub fn process_command(message: &[u8], data: &json::JsonValue) -> json::JsonValue{
    let msg = str::from_utf8(&message).unwrap();
    let cmds: Vec<&str> = msg.split(CMD_DELIMITER).collect();

    match 1 {
        1 | _ if HELLO.is_match(cmds[0]) => {
            match cmds[1] {
                "from" => {
                    let mut o = data.clone();
                    let mut x = "hello from ".to_string();
                    x.push_str(cmds[2]);
                    let dat = x.as_bytes();
                    let new_data = update_datapoint!(o, dat);

                    new_data
                },
                _ => {
                    data.clone()
                }
            }
        },
        _ => {
            data.clone()
        }
    }
}
