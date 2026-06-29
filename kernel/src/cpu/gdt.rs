use x86_64::registers::segmentation::{Segment, CS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

struct Selectors {
    code_selector: SegmentSelector,
}

static GDT: spin::Lazy<(GlobalDescriptorTable, Selectors)> = spin::Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::empty();
    let code_selector = gdt.append(Descriptor::kernel_code_segment());

    (gdt, Selectors { code_selector })
});

pub fn init() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
    }
}