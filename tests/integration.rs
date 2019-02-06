#[macro_use]
extern crate dotlib;
extern crate json;

/// UINTEGRATION TESTING
#[cfg(test)]
mod tests {
    use std::error::Error;
    use dotlib::helper::{CMD_DELIMITER, Events, DataDotError};
    use json::JsonValue;
    use std::net::{IpAddr, Ipv4Addr};
    use dotlib::dot::Dot;

    #[test]
    fn test_delimiter(){
        assert_eq!(CMD_DELIMITER, "::".to_string());
    }

    #[test]
    fn test_trait_events(){
        struct Tmp {}
        impl Events for Tmp{}

        let t = Tmp{};
        let o = JsonValue::new_object();
        let c = JsonValue::new_object();
        let rslt1: Result<String, DataDotError> = Err(DataDotError::new("No data available"));
        let rslt2: Result<String, DataDotError> = Err(DataDotError::new("No data available"));

        assert_eq!(t.after_delete(&o.clone()), c);
        assert_eq!(t.before_delete(&o.clone()), c);
        
        assert_eq!(t.after_get(&o.clone()), c);
        assert_eq!(t.before_get(&o.clone()), c);
                
        assert_eq!(t.after_post(&o.clone()), c);
        assert_eq!(t.before_post(&o.clone()), c);

        assert_eq!(t.after_put(&o.clone()), c);
        assert_eq!(t.before_put(&o.clone()), c);

        assert_eq!(t.on_error(rslt1.err().unwrap()),rslt2.err().unwrap());

        assert_eq!(t.on_load_plugin(&o.clone()), c);
        assert_eq!(t.on_unload_plugin(&o.clone()), c);

        assert_eq!(t.on_shutdown(&o.clone()), c);

        assert_eq!(t.process_command("test message".to_string().as_bytes(), &o.clone()), JsonValue::new_object());
    }

    #[test]
    fn test_error_handling(){
        let rslt: Result<String, DataDotError> = Err(DataDotError::new("No data available"));

        match rslt {
            Ok(_v) => assert!(false),
            Err(e) => assert_eq!(e.description(), "No data available")
        }
    }

    #[test]
    fn test_plugin_hello(){
        let a: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
        let p: u16 = 7645;
        let path = Some("./target/debug/examples");
        let mut dot = Dot::new(a, p, String::from("hello").as_bytes(), path);

        assert!(dot.process_command("HELLO::from::me".as_bytes()).is_ok());
        assert_eq!(dot.get_datapoint(),"hello from me".to_string().as_bytes());
    }

    #[test]
    fn test_macro_update_datapoint(){
        let data = JsonValue::new_object();
        let x = "my data".to_string();
        let dat = x.as_bytes();
        let new_data = update_datapoint!(data, dat);
        let mut arr: Vec<u8> = Vec::new();

        for byt in new_data["data"].members(){
            arr.push(byt.as_u8().unwrap().clone());
        }

        assert_eq!(arr, "my data".to_string().as_bytes());

        let x = "my new data".to_string();
        let dat = x.as_bytes();
        let new_data = update_datapoint!(data, dat);

        let mut arr: Vec<u8> = Vec::new();

        for byt in new_data["data"].members(){
            arr.push(byt.as_u8().unwrap().clone());
        }

        assert_eq!(arr, "my new data".to_string().as_bytes());
    }
}
