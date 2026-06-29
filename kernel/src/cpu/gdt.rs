use crate::cpu::tss::TSS;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

static GDT: spin::Lazy<(GlobalDescriptorTable, Selectors)> = spin::Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::empty();
    let code_selector = gdt.append(Descriptor::kernel_code_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));

    (gdt, Selectors { code_selector, tss_selector })
});

pub fn init() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}