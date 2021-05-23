use std::mem::MaybeUninit;

use crate::isolate::Isolate;

extern "C" {
    fn v8__Locker__CONSTRUCT(buf: *mut MaybeUninit<Locker>, isolate: *mut Isolate, custom_archive: *mut libc::c_void);
    fn v8__Locker__DESTRUCT(this: *mut Locker, custom_archive: *mut libc::c_void);
    fn v8__UnLocker__CONSTRUCT(buf: *mut MaybeUninit<UnLocker>, isolate: *mut Isolate);
    fn v8__UnLocker__DESTRUCT(this: *mut UnLocker);
}

#[repr(C)]
pub struct Locker([usize; 2], *mut libc::c_void);

impl Locker {
    pub fn new(isolate: &mut Isolate, custom_archive: *mut libc::c_void) -> Self {
        let mut locker: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe {
            v8__Locker__CONSTRUCT(&mut locker, isolate, custom_archive);
            let mut l = locker.assume_init();
            l.1 = custom_archive;
            l
        }
    }
}

impl Drop for Locker {
    fn drop(&mut self) {
        unsafe { v8__Locker__DESTRUCT(self, self.1) };
    }
}

#[repr(C)]
pub struct UnLocker([usize; 1]);
impl UnLocker {
    pub fn new(isolate: &mut Isolate) -> Self {
        let mut unlocker: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe {
            v8__UnLocker__CONSTRUCT(&mut unlocker, isolate);
            let l = unlocker.assume_init();
            l
        }
    }
}
impl Drop for UnLocker {
    fn drop(&mut self) {
        unsafe { v8__UnLocker__DESTRUCT(self) };
    }
}


pub struct IsolateScope(*mut Isolate);
impl IsolateScope {
    pub fn new(isolate: &mut Isolate) -> Self {
        unsafe { isolate.enter() };
        Self(isolate)
    }
}

impl Drop for IsolateScope {
    fn drop(&mut self) {
        let isolate = unsafe { &mut *self.0 };
        unsafe { isolate.exit() };
    }
}
