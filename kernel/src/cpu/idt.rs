use x86_64::structures::idt::InterruptDescriptorTable;

static IDT: spin::Lazy<InterruptDescriptorTable> = spin::Lazy::new(InterruptDescriptorTable::new);

pub fn init() {
    IDT.load();
}