//! A safe and lightweight real-time Kernel written in Rust. The Kernel is developed for cortex-m3/m4 
//! based microcontrollers. The goal of the project was to develop a memory efficient, 
//! safe and lightweight real-time Kernel. Rust-lang was choosen due to its powerful compile-time checks.
//!  All the subsystems have been developed and manually tested. Language features like Traits, Generics 
//! helped define powerful and safe types. The Kernel uses statically allocated data structures to keep 
//! itself simple and fast. But using feature flags, the developer can enable dynamic memory allocation 
//! (for end-application and not Kernel itself). Cargo feature flags are used to configure constants 
//! such as maximum tasks, resources, etc. 
//! 
//! Features Flags : 
//! 
//! * alloc : Enables use of dynamic data-structures.
//! * tasks_8 : Max tasks in Kernel is set to 8.
//! * tasks_16 : Max tasks in Kernel is set to 16.
//! * tasks_32 : Max tasks in Kernel is set to 32.
//! * resources_16 : Max resources in Kernel is set to 16.
//! * resources_32 : Max resources in Kernel is set to 32.
//! * resources_64 : Max resources in Kernel is set to 64.
//! * messages_16 : Max messages in Kernel is set to 16.
//! * messages_32 : Max messages in Kernel is set to 32.
//! * messages_64 : Max messages in Kernel is set to 64.
//! * events_16 : Max events in Kernel is set to 16.
//! * events_32 : Max events in Kernel is set to 32.
//! * events_64 : Max events in Kernel is set to 64.

#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(const_if_match)]
#![feature(const_loop)]
#![feature(const_generics)]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "alloc")]
extern crate alloc_cortex_m;

#[allow(non_upper_case_globals)]

#[macro_use]
extern crate cortex_m_rt;

mod config;
mod kernel;
mod system;
mod utils;

pub mod macros;

use crate::utils::errors::KernelError;

/// Helper functions.
pub mod helpers {
    pub use crate::utils::helpers::TaskMask;
    pub use crate::utils::helpers::is_privileged;
}

/// Kernel routines which assist in Event management.
#[cfg(any(feature = "events_32", feature = "events_16", feature = "events_64"))]
pub mod events {
    pub use crate::kernel::events::enable;
    pub use crate::kernel::events::disable;
    pub use crate::kernel::events::new;
}

#[cfg(feature = "timer")]
pub mod timer {
    pub use crate::kernel::timer::start_timer;
}
/// Kernel routines which assist in Inter-task Communication.
pub mod primitives {
    pub use crate::system::message::Message;
    pub use crate::system::resource::*;
    pub use crate::system::semaphore::Semaphore;
}

/// Kernel routines which assist in Task management.
pub mod tasks {
    pub use crate::kernel::tasks::create_task;
    pub use crate::kernel::tasks::init;
    pub use crate::kernel::tasks::release;
    pub use crate::kernel::tasks::start_kernel;
    pub use crate::kernel::tasks::task_exit;
}

#[cfg(feature="system_logger")]
pub mod logging {
    pub use crate::kernel::logging::process;
    pub use crate::kernel::logging::set_all;
    pub use crate::kernel::logging::set_release;
    pub use crate::kernel::logging::set_block_tasks;
    pub use crate::kernel::logging::set_unblock_tasks;
    pub use crate::kernel::logging::set_task_exit;
    pub use crate::kernel::logging::set_resource_lock;
    pub use crate::kernel::logging::set_resource_unlock;
    pub use crate::kernel::logging::set_message_broadcast;
    pub use crate::kernel::logging::set_message_recieve;
    pub use crate::kernel::logging::set_semaphore_signal;
    pub use crate::kernel::logging::set_semaphore_reset;
    pub use crate::kernel::logging::set_timer_event;
    pub use crate::system::system_logger::LogEvent;
}

#[cfg(feature = "alloc")]
pub use crate::utils::heap;
