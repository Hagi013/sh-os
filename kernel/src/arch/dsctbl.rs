use super::asmfunc;

const ADR_GDT: u32 = 0x00270000;
const LIMIT_GDT: usize = 0x0000ffff;
const ADR_OSPAK: u32 = 0x00280000;
const LIMIT_OSPAK: u32 = 0x0007ffff;
const AR_DATA32_RW: u32 = 0x4092;
const AR_CODE32_ER: u32 = 0x409a;

const ADR_IDT: u32 = 0x0026f800;
const LIMIT_IDT: usize = 0x000007ff;

struct SegmentDescriptorEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_right: u8,
    limit_high: u8,
    base_high: u8,
}

struct SegmentDescriptorTable {
    base_pointer: u32,
}

struct GateDescriptorEntry {
    offset_low: u16,
    selector: u16,
    dw_count: u8,
    access_right: u8,
    offset_high: u8,
}

pub struct DscTbl {
    segment_descriptor_table: [*mut SegmentDescriptorEntry; LIMIT_GDT / 8],
    gate_descriptor_table: [*mut GateDescriptorEntry; LIMIT_IDT / 8],
}

impl DscTbl {
    fn new() -> Self {
        DscTbl {
            segment_descriptor_table: DscTbl::init_gdt(),
            gate_descriptor_table: DscTbl::init_idt(),
        }
    }

    pub fn init_gdt_idt() -> Self {
        return DscTbl::new();
    }

    fn init_gdt() -> [*mut SegmentDescriptorEntry; LIMIT_GDT / 8] {
        let mut segment_descriptor_table: [*mut SegmentDescriptorEntry; LIMIT_GDT / 8] = [0 as *mut SegmentDescriptorEntry; LIMIT_GDT / 8];
        (0..(LIMIT_GDT / 8))
            .map(|idx| DscTbl::set_segmdesc(idx as u32, 0, 0, 0))
            .zip(segment_descriptor_table.iter_mut())
            .for_each(|(b, df)| *df = b);

        segment_descriptor_table[1] = DscTbl::set_segmdesc(1, 0xffffffff, 0x00000000, AR_DATA32_RW);
        segment_descriptor_table[2] = DscTbl::set_segmdesc(2, LIMIT_OSPAK, ADR_OSPAK, AR_CODE32_ER);

        asmfunc::load_gdtr(LIMIT_GDT as u32, ADR_GDT);

        return segment_descriptor_table
    }

    fn set_segmdesc(idx: u32, mut limit: u32, base: u32, mut ar: u32) -> *mut SegmentDescriptorEntry {
        if limit > 0xfffff {
            ar |= 0x8000; /* G_bit = 1 */
            limit /= 0x1000;
        }

        let base_point: u32 = unsafe { *(ADR_GDT as *const u32) };
        let base_address: *mut u32 = (base_point + (idx * 8)) as *mut u32;
        let seg_dsc_entry: *mut SegmentDescriptorEntry = base_address as *mut SegmentDescriptorEntry;
        unsafe {
            (*seg_dsc_entry).limit_low = (limit & 0xffff) as u16;
            (*seg_dsc_entry).base_low = (base & 0xffff) as u16;
            (*seg_dsc_entry).base_mid = ((base >> 16) & 0xff) as u8;
            (*seg_dsc_entry).access_right = (ar & 0xff) as u8;
            (*seg_dsc_entry).limit_high = (((limit >> 16) & 0xff) | ((ar >> 8) & 0xf0)) as u8;
            (*seg_dsc_entry).base_high = ((base >> 24) & 0xff) as u8;
        }
        return seg_dsc_entry;
    }

    fn init_idt() -> [*mut GateDescriptorEntry; LIMIT_IDT / 8] {
        let mut gate_descriptor_table: [*mut GateDescriptorEntry; LIMIT_IDT / 8] = [0 as *mut GateDescriptorEntry; LIMIT_IDT / 8];
        (0..(LIMIT_IDT / 8))
            .map(|idx| DscTbl::set_gatedesc(idx as u32, 0, 0, 0))
            .zip(gate_descriptor_table.iter_mut())
            .for_each(|(b, df)| *df = b);

        asmfunc::load_idtr(LIMIT_IDT as u32, ADR_IDT);
        return gate_descriptor_table
    }

    fn set_gatedesc(idx: u32, offset: u32, selector: u32, ar: u32) -> *mut GateDescriptorEntry {
        let base_point: u32 = unsafe { *(ADR_IDT as *const u32) };
        let base_address: *mut u32 = (base_point + (idx * 8)) as *mut u32;
        let gate_dsc_entry: *mut GateDescriptorEntry = base_address as *mut GateDescriptorEntry;
        unsafe {
            (*gate_dsc_entry).offset_low = (offset & 0xffff) as u16;
            (*gate_dsc_entry).selector = selector as u16;
            (*gate_dsc_entry).dw_count = ((ar >> 8) & 0xff) as u8;
            (*gate_dsc_entry).access_right = (ar & 0xff) as u8;
            (*gate_dsc_entry).offset_high = ((offset >> 16) & 0xffff) as u8;
        }
        return gate_dsc_entry;
    }
}
