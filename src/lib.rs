#![no_std]

extern crate fallible;
extern crate null_terminated;

use core::fmt;
use core::mem;
use fallible::*;
use null_terminated::*;

pub mod arch;

#[cfg(target_arch = "x86_64")]
pub use arch::x86_64::{Frame, Regs as Cxt};

#[link(name = "unwind")]
extern "C" {
    #[link_name = "unw_getcontext"]
    fn c_getcxt(_: *mut Cxt) -> isize;

    #[link_name = "unw_init_local"]
    fn c_init_local(_: *mut Frame, _: *const Cxt) -> isize;

    #[link_name = "unw_step"]
    fn c_step(_: *mut Frame) -> isize;

    #[link_name = "unw_resume"]
    fn c_resume(_: *const Frame) -> isize;

    #[link_name = "unw_strerror"]
    fn c_strerror(_: usize) -> *const u8;

    #[link_name = "unw_get_proc_name"]
    fn c_get_proc_name(_: *const Frame, _: *mut u8, _: usize, _: *mut usize) -> isize;
}

impl Cxt {
    #[inline(always)]
    pub fn new() -> Result<Self, Error> { unsafe {
        let mut cxt = mem::uninitialized();
        match c_getcxt(&mut cxt) {
            0 => Ok(cxt),
            e => Err(Error(-e as _)),
        }
    } }
}

impl Frame {
    #[inline]
    pub unsafe fn jump(&self) -> Error { Error(-c_resume(self) as _) }

    #[inline]
    pub fn next(mut self) -> Result<Option<Self>, Error> {
        let c = unsafe { c_step(&mut self) };
        use core::cmp::Ordering::*;
        match isize::cmp(&c, &0) {
            Less    => Err(Error(-c as _)),
            Greater => Ok(Some(self)),
            Equal   => Ok(None),
        }
    }

    #[inline]
    pub fn loc<A, F: FnOnce(&[u8], usize) -> A>(self, f: F) -> Result<A, Error> { unsafe {
        let mut bs: [u8; 4096] = mem::uninitialized();
        let mut offset: usize = mem::uninitialized();
        match c_get_proc_name(&self, &mut bs[0], bs.len(), &mut offset) {
            0 => Ok(f(&Nul::new_unchecked(&bs[0])[..], offset)),
            e => Err(Error(-e as _)),
        }
    } }
}

impl TryFrom<Cxt> for Frame {
    type Error = Error;
    #[inline]
    fn try_from(cxt: Cxt) -> Result<Self, Error> { unsafe {
        let mut frame = mem::uninitialized();
        match c_init_local(&mut frame, &cxt) {
            0 => Ok(frame),
            e => Err(Error(-e as _)),
        }
    } }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Error(usize);

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::str;
        f.write_str(unsafe {
            str::from_utf8_unchecked(&Nul::new_unchecked(c_strerror(self.0))[..])
        })
    }
}
