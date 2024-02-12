use core::arch::asm;

use lazy_static::lazy_static;
use log::info;
use crate::{config::*, shutdown};
use crate::{sync::up::UPSafeCell, trap::context::TrapContext};

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE]
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE]
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack { data: [0; KERNEL_STACK_SIZE] }; MAX_APP_NUM];
static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack { data: [0; USER_STACK_SIZE] }; MAX_APP_NUM];

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

lazy_static!{
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" { fn _num_app(); }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start
            }
        })
    };
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!("[kernel] app_{} [{:#x}, {:#x})", i, self.app_start[i], self.app_start[i+1]);
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}


pub fn init() {
    load_apps();
    print_app_info();
}

pub fn get_current_app() -> usize {
    APP_MANAGER.exclusive_access().current_app
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn get_current_app_memory_range() -> [usize; 2] {
    let app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    drop(app_manager);
    [get_base_i(current_app), get_base_i(current_app)+APP_SIZE_LIMIT]
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    if current_app >= app_manager.num_app {
        info!("[kernel] All application finished!");
        shutdown(false);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK[0].push_context(TrapContext::app_init_context(
            get_base_i(current_app), USER_STACK[current_app].get_sp())) as *const _ as usize);
    }
    unreachable!("Unreachable run_next_app code");
}

pub fn load_apps() {
    extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = unsafe {
        num_app_ptr.read_volatile()
    };
    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
    };
    // clear cache
    unsafe {
        asm!("fence.i");
    }
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear memory
        unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, APP_SIZE_LIMIT).fill(0);
        }
        // load app
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i+1] - app_start[i])
        };

        let dst = unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, src.len())
        };
        dst.copy_from_slice(src);
    }

}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}