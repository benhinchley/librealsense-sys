#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_devices() {
        unsafe {
            let err = Box::into_raw(Box::new(rs_error([]))) as *mut *mut rs_error;
            let api: i32 = RS_API_VERSION as i32;
            let ctx = rs_create_context(api, err);
            let num = rs_get_device_count(ctx, err);
            println!("connected devices: {:?}", num);
        }
    }
}
