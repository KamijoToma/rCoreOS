use _core::mem::size_of;
use _core::slice::from_raw_parts;
use alloc::{string::String, vec};
use alloc::vec::Vec;
use bitflags::*;



use super::{
    address::{PhysPageNum, StepByOne, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
};

bitflags! {
    #[derive(Default)]
    pub struct PTEFlags: u8 {
        const V = 1 << 0; // 合法标志
        const R = 1 << 1; // 可读标志
        const W = 1 << 2; // 可写标志
        const X = 1 << 3; // 可执行标志
        const U = 1 << 4; // U特权级是否允许访问标志
        const G = 1 << 5; // Unknown
        const A = 1 << 6; // 访问标志
        const D = 1 << 7; // 修改标志
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        } // mix together
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap_or_default()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty() // V 标志位
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Option<Self> {
        frame_alloc().map(|frame| PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        })
    }
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, item) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*item];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V); // 新页表
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, item) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*item];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                // 不处理虚拟页面为空的问题
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    // 临时手动查找页表
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    pub fn token(&self) -> usize {
        // satp寄存器格式要求 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter4/3sv39-implementation-1.html#satp-layout
        8usize << 60 | self.root_ppn.0
    }

    pub fn translate_va(&self, va: VirtAddr) -> Option<&'static mut u8> {
        let vpn = va.floor();
        if let Some(ppe) = self.translate(vpn) {
            if !ppe.readable() {
                return None
            }
            let ppn = ppe.ppn();
            return Some(&mut ppn.get_bytes_array()[va.page_offset()])
        }
        None
    }
}

pub fn translated_byte_buffer_mut(token: usize, ptr: *const u8, len: usize) -> Option<Vec<&'static mut [u8]>> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppe = page_table.translate(vpn).unwrap();
        if !ppe.readable() || !ppe.writable() {
            return None
        }
        let ppn = ppe.ppn();
        vpn.step();
        let mut end_va = VirtAddr::from(vpn);
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..])
        } else {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    Some(v)
}

pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Option<Vec<&'static [u8]>> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppe = page_table.translate(vpn).unwrap();
        if !ppe.readable() {
            return None
        }
        let ppn = ppe.ppn();
        vpn.step();
        let mut end_va = VirtAddr::from(vpn);
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&ppn.get_bytes_array()[start_va.page_offset()..])
        } else {
            v.push(&ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    Some(v)
}

pub fn translate_str(token: usize, ptr: *const u8) -> Option<String> {
    let page_table = PageTable::from_token(token);
    let mut string = String::new();
    let mut start = ptr as usize;
    loop {
        if let Some(ch) = page_table.translate_va(VirtAddr::from(start)) {
            if *ch == 0 {
                break;
            } else {
                string.push(*ch as char);
                start += 1;
            }
        }else{
            return None;
        }
    }
    Some(string)
}

pub fn translate_memcopy<T>(token: usize, ptr: *const u8, source: &T) -> Result<(), ()> where T: Sized{
    if let Some(dst_list) = translated_byte_buffer_mut(token, ptr, size_of::<T>()){
        unsafe {
            let src = from_raw_parts(
                source as *const T as *const u8,
                size_of::<T>(),
            );
            let mut start = 0usize;
            let mut end = 0usize;
            for dst in dst_list {
                end += dst.len();
                dst.copy_from_slice(&src[start..end]);
                start = end;
            }
        }
        Ok(())
    } else {
        Err(())
    }

}