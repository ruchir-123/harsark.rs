//! Interrupt Handlers
use cortex_m_rt::exception;

use cortex_m::interrupt::free as execute_critical; 
use crate::kernel::tasks::{TaskManager};
use crate::kernel::tasks::schedule;
use crate::utils::arch::return_to_psp;

#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
use crate::kernel::events::sweep_event_table;

#[cfg(feature="process_monitor")]
use crate::kernel::process_monitor::sweep_deadlines;

use crate::kernel::timer::update_time;
/// ### SysTick Interrupt handler
/// Its the Crux of the Kernel’s time management module and Task scheduling.
/// This interrupt handler updates the time and also dispatches the appropriate event handlers.
/// The interrupt handler also calls `schedule()` in here so as to dispatch any higher priority
/// task if there are any.

use cortex_m_semihosting::hprintln;
#[cfg(feature="timer")]
#[exception]
fn SysTick() {

    #[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
    sweep_event_table();

    #[cfg(feature="timer")]
    update_time();
    
    #[cfg(feature="process_monitor")]
    sweep_deadlines();
    
    // hprintln!("hello");
    schedule();
}
/// ### SVC Interrupt handler,
/// calls `tasks::schedule()`
#[exception]
fn SVCall() {
    schedule();
}
/// ### PendSV Interrupt handler,
/// PendSV interrupt handler does the actual context switch in the Kernel.
#[exception]
fn PendSV() {
    execute_critical(|cs_token| {
        let handler = &mut TaskManager.borrow(cs_token).borrow_mut();
        let curr_tid: usize = handler.curr_tid;
        let next_tid: usize = handler.get_next_tid() as usize;
        if curr_tid != next_tid || (!handler.started) {
            if handler.started {
                let curr_task = handler.task_control_blocks[curr_tid].as_ref().unwrap();
                curr_task.save_context();
            } else {
                handler.started = true;
            }
            let next_task = handler.task_control_blocks[next_tid].as_ref().unwrap();
            next_task.load_context();
    
            handler.curr_tid = next_tid;
        }
    });
    return_to_psp()
}
