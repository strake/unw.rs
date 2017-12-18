use core::hash::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Regs {
    pub rax: usize, pub rbx: usize, pub rcx: usize, pub rdx: usize,
    pub rdi: usize, pub rsi: usize, pub rbp: usize, pub rsp: usize,
    pub r8 : usize, pub r9 : usize, pub r10: usize, pub r11: usize,
    pub r12: usize, pub r13: usize, pub r14: usize, pub r15: usize,
    pub rip: usize, pub rflags: usize,
    pub cs: usize, pub fs: usize, pub gs: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Frame([usize; 33]);

impl PartialEq for Frame {
    #[inline] fn eq(&self, other: &Self) -> bool { self.0[..] == other.0[..] }
}

impl Eq for Frame {}

impl Hash for Frame {
    #[inline] fn hash<H: Hasher>(&self, h: &mut H) { self.0[..].hash(h) }
}
