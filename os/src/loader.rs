use core::{arch::asm, slice::from_raw_parts};

use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::info;

use crate::console::print;

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    unsafe { num_app_ptr.read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    unsafe {
        // clear cache
        asm!("fence.i");
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

lazy_static! {
    static ref APP_NAMES: Vec<&'static str> = {
        let num_app = get_num_app();
        extern "C" { fn _app_names(); }
        let mut start = _app_names as usize as *const u8;
        let mut v = Vec::new();
        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                while end.read_volatile() != '\0' as u8 {
                    end = end.add(1);
                } // 加载字符串
                let str = core::str::from_utf8(
                    from_raw_parts(start, end as usize - start as usize)
                ).unwrap();
                v.push(str);
                start = end.add(1);
            }
        }
        v
    };
}

pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let num_app = get_num_app();
    (0..num_app)
        .find(|&i| APP_NAMES[i] == name)
        .map(|i| get_app_data(i))
}

pub fn list_apps() {
    info!("- Available apps are: ");
    APP_NAMES.iter().enumerate().for_each(|(i, app_name)| {
        print!(" {},", app_name);
        if (i+1)%4 == 0 {
            println!("");
        }
    });
    info!("---------- Avail Apps -----------");
}