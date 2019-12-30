//! # Machine specific
//!
//! Defines functions which are defined majorly in assembly. Thus, might change for one board to another.

use cortex_m::interrupt::free as execute_critical; 
use crate::kernel::task_management::{all_tasks,os_curr_task_id,os_next_task_id};

/// Returns the MSB of `val`. It is written using CLZ instruction.
pub fn get_msb(val: u32) -> usize {
    let mut res;
    unsafe {
        asm!("clz $1, $0"
        : "=r"(res)
        : "0"(val)
        );
    }
    res = 32 - res;
    if res > 0 {
        res -= 1;
    }
    return res;
}

/// Returns true if Currently the Kernel is operating in Privileged mode.
pub fn is_privileged() -> bool {
    let val: u32;
    unsafe {
        asm!("mrs $0, CONTROL"
            : "=r"(val)
            :
        )
    };
    !((val & 1) == 1)
}

/// Creates an SVC Interrupt
pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}

/// PendSV interrupt handler does the actual context switch in the Kernel.
pub fn pendSV_handler() {
        execute_critical(|cs_token| {
            let curr_tid: usize = unsafe { os_curr_task_id };
            let next_tid: usize = unsafe { os_next_task_id };
            let handler = &mut all_tasks.borrow(cs_token).borrow_mut();
            
            if handler.started {
                let curr_task = handler.task_control_blocks[curr_tid].as_ref().unwrap();
                curr_task.save_context();
            } else {
                handler.started = true;
            }
            
            let next_task = handler.task_control_blocks[next_tid].as_ref().unwrap();
            next_task.load_context();
        });

        unsafe{
            asm!("
            ldr r0, =0xFFFFFFFD
            bx	r0
            ");
        }
            
}