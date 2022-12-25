#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const fn XPR_RTSP_PORT(major: u32, minor: u32, track: u32) -> u32 {
    (major << 24) | (minor << 8) | track
}

pub const fn XPR_RTSP_PORT_MAJOR(port: u32) -> u32 {
    (port >> 24) & 0x0000007f
}

pub const fn XPR_RTSP_PORT_MINOR(port: u32) -> u32 {
    (port >> 8) & 0x0000ffff
}

pub const fn XPR_RTSP_PORT_STREAM(port: u32) -> u32 {
    (port >> 8) & 0x0000ffff
}

pub const fn XPR_RTSP_PORT_TRACK(port: u32) -> u32 {
    port & 0x000000ff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtsp_open_close() {
        unsafe {
            XPR_RTSP_Init();
            let mgrPort = XPR_RTSP_PORT(XPR_RTSP_PORT_MAJOR_CLI, 0, 0) as i32;
            let ret = XPR_RTSP_Open(mgrPort, b"dummy://any\0".as_ptr());
            assert_eq!(ret, 0);
            let ret = XPR_RTSP_Start(mgrPort);
            assert_eq!(ret, 0);
            let ret = XPR_RTSP_Stop(mgrPort);
            assert_eq!(ret, 0);
            XPR_RTSP_Close(mgrPort);
            XPR_RTSP_Fini();
        }
    }
}
