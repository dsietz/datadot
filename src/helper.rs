use std::error::Error;
use std::fmt;
use std::cmp::PartialEq;
use std::any::Any;
use json::object::Object;
use json::JsonValue;

pub const CMD_DELIMITER: &str = "::";

#[allow(unused_variables)]
pub trait Events {
    fn after_delete(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn after_get(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn after_post(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn after_put(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn before_delete(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn before_get(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn before_post(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn before_put(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn on_error(&self, err: DataDotError) -> DataDotError{
        err
    }

    fn on_load_plugin(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn on_shutdown(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn on_unload_plugin(&self, data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }

    fn process_command(&self, message: &[u8], data: &json::JsonValue) -> json::JsonValue{
        data.clone()
    }
}

#[derive(Debug)]
pub struct DataDotError {
    details: String
}

impl DataDotError {
    pub fn new(msg: &str) -> DataDotError {
        DataDotError{
            details: msg.to_string()
        }
    }
}

impl fmt::Display for DataDotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DataDotError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl PartialEq for DataDotError {
    fn eq(&self, other: &DataDotError) -> bool {
        self.description() == other.description()
    }
}