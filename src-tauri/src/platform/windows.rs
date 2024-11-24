use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, SystemParametersInfoW,
    SPI_GETACTIVEWINDOWTRACKING, SPI_GETACTIVEWNDTRKTIMEOUT,
    SPI_SETACTIVEWINDOWTRACKING, SPI_SETACTIVEWNDTRKTIMEOUT,
    SPIF_SENDCHANGE,
};

use super::MainModule;

pub struct Windows {
    orgin_window_tracking_config: bool,
    orgin_window_tracking_delay_config: u64,
}

impl MainModule for Windows {
    fn initialize(&mut self) {
        self.orgin_window_tracking_config =
            self.get_window_tracking_config();
        self.orgin_window_tracking_delay_config =
            self.get_window_tracking_delay_config();

        self.set_window_tracking_config(true);
        self.set_window_tracking_delay_config(100);
    }
    fn finialize(&mut self) {
        self.set_window_tracking_config(
            self.orgin_window_tracking_config
        );
        self.set_window_tracking_delay_config(
            self.orgin_window_tracking_delay_config
        );
    }

    fn focus_on(&self) {
        if self.get_window_tracking_config() {
            return;
        }
        self.set_window_tracking_config(true);
    }
    fn focus_off(&self) {
        if !self.get_window_tracking_config() {
            return;
        }
        self.set_window_tracking_config(false);
    }
}

impl Windows {
    pub fn new() -> Self {
        Windows {
            orgin_window_tracking_config: false,
            orgin_window_tracking_delay_config: 0,
        }
    }

    fn get_window_tracking_config(&self) -> bool {
        let mut enable = 0;

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_GETACTIVEWINDOWTRACKING,
                0,
                Some(&mut enable as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };

        match ret {
            Ok(_) => {}
            Err(_) => { panic!("Failed to get window tracking"); }
        }

        enable == 1
    }
    fn set_window_tracking_config(&self, enable: bool) {
        let enable = if enable { 1 } else { 0 };

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_SETACTIVEWINDOWTRACKING,
                0,
                Some(enable as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };
        match ret {
            Ok(_) => {}
            Err(_) => { panic!("Failed to set window tracking"); }
        }
    }

    fn get_window_tracking_delay_config(&self) -> u64 {
        let mut delay = 0;

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_GETACTIVEWNDTRKTIMEOUT,
                0,
                Some(&mut delay as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };

        match ret {
            Ok(_) => {}
            Err(_) => { panic!("Failed to get window tracking delay"); }
        }

        delay as u64
    }
    fn set_window_tracking_delay_config(&self, delay_msec: u64) {
        let ret = unsafe {
            SystemParametersInfoW(
                SPI_SETACTIVEWNDTRKTIMEOUT,
                0,
                Some(delay_msec as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };
        match ret {
            Ok(_) => {}
            Err(_) => { panic!("Failed to set window tracking delay"); }
        }
    }

    fn get_cursor_pos(&self) -> POINT {
        let mut pos = POINT { x: 0, y: 0 };

        let ret = unsafe { GetCursorPos(&mut pos) };
        match ret {
            Ok(_) => {}
            Err(_) => { panic!("Failed to get cursor position"); }
        }

        pos
    }
}
