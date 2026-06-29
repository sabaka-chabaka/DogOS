use core::ptr::addr_of;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub static TSS: spin::Lazy<TaskStateSegment> = spin::Lazy::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe {addr_of!(STACK)});
        stack_start + STACK_SIZE as u64
    };

    tss
});
