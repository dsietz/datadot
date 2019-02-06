#[macro_export]
macro_rules! update_datapoint {
    ( $d: ident, $u: ident ) => {
        {
            use json::JsonValue;

            let mut data = $d.clone();
            let mut dat= json::JsonValue::new_array();

            for byt in $u{
                dat.push(*byt as u64);
            }

            data["data"] = dat;
            data.clone()
        }
    }
}