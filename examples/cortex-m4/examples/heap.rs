#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

extern crate panic_halt;
extern crate stm32f4;

use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use alloc::vec::Vec;

use hartex_rust::alloc;
use hartex_rust::heap::init_heap;
use hartex_rust::tasks::*;
use hartex_rust::helpers::TaskMask;
use hartex_rust::primitives::*;
use hartex_rust::spawn;

/*
lazy_static is used to define global static variables.

Declaring variables in lazy_static can be useful while sharing kernel primitives to kernel tasks and interrupt
handlers. Resources can be shared to tasks as a parameter but interrupt handlers do not take parameters, hence
only way to share data with them is via global statics.

The Resource res1 stores a resource of type Vec. Vec is a dynamic memory data structure.
*/

const task1: u32 = 1;
const task2: u32 = 2;


#[entry]
fn main() -> ! {
    // Initialize heap for the application. The argument is the size of the heap.
    init_heap(1024);
    let peripherals = init_peripherals();
    
    static mut stack1: [u32; 1024] = [0; 1024];
    static mut stack2: [u32; 1024] = [0; 1024];
    
    static resource1: Resource<RefCell<Vec<u32>>> = Resource::new(RefCell::new(Vec::new()), TaskMask::generate([1, 2]));
    
    spawn!(task1, stack1, {
        hprintln!("TASK 1: Enter");
        resource1.acquire(|res| {
            let res = &mut res.borrow_mut();
            res.push(1);
            hprintln!("TASK 1: Resource : {:?}", res);
        });
        hprintln!("TASK 1: End");
    });
    spawn!(task2, stack2, {
        hprintln!("TASK 2: Enter");
        resource1.acquire(|res| {
            let res = &mut res.borrow_mut();
            res.push(2);
            hprintln!("TASK 2: Resource : {:?}", res);
        });
        hprintln!("TASK 2: End");
    });

    init();
    release(TaskMask::generate([task1, task2]));
    start_kernel()
}
