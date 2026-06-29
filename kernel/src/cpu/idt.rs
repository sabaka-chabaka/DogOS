use crate::console;
use crate::cpu::tss;
use crate::drivers::serial;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static IDT: spin::Lazy<InterruptDescriptorTable> = spin::Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.general_protection_fault
        .set_handler_fn(general_protection_fault_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);

    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(tss::DOUBLE_FAULT_IST_INDEX);
    }

    idt
});

pub fn init() {
    IDT.load();
}

fn unlock_outputs() {
    unsafe {
        console::force_unlock();
        serial::force_unlock();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    unlock_outputs();
    log::warn!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    unlock_outputs();
    log::error!(
        "EXCEPTION: DOUBLE FAULT (error code {})\n{:#?}",
        error_code,
        stack_frame
    );
    loop {}
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    unlock_outputs();
    log::error!(
        "EXCEPTION: GENERAL PROTECTION FAULT (error code {})\n{:#?}",
        error_code,
        stack_frame
    );
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let accessed_address = x86_64::registers::control::Cr2::read();
    unlock_outputs();
    log::error!(
        "EXCEPTION: PAGE FAULT\naccessed address: {:?}\nerror code: {:?}\n{:#?}",
        accessed_address,
        error_code,
        stack_frame
    );
    loop {}
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    unlock_outputs();
    log::error!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
    loop {}
}