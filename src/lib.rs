#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl rs_error {
    pub fn new() -> rs_error {
        rs_error([])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_devices() {
        unsafe {
            let mut err = Box::into_raw(Box::new(rs_error::new())) as *mut rs_error;
            let api: i32 = RS_API_VERSION as i32;
            let ctx = rs_create_context(api, &mut err as *mut _);
            let num = rs_get_device_count(ctx, &mut err as *mut _);
            assert!(num >= 0);
        }
    }
}
