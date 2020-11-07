use std::mem::MaybeUninit;

use crate::isolate::Isolate;

extern "C" {
    fn v8__Locker__CONSTRUCT(buf: *mut MaybeUninit<Locker>, isolate: *mut Isolate);
    fn v8__Locker__DESTRUCT(this: *mut Locker);
}

#[repr(C)]
pub struct Locker([usize; 2]);

impl Locker {
    pub fn new(isolate: &mut Isolate) -> Self {
        let mut locker: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe {
            v8__Locker__CONSTRUCT(&mut locker, isolate);
            locker.assume_init()
        }
    }
}

impl Drop for Locker {
    fn drop(&mut self) {
        unsafe { v8__Locker__DESTRUCT(self) };
    }
}

pub struct IsolateScope(*mut Isolate);
impl IsolateScope {
    pub fn new(isolate: &mut Isolate) -> Self {
        isolate.enter_isolate();
        Self(isolate)
    }
}

impl Drop for IsolateScope {
    fn drop(&mut self) {
        let isolate = unsafe { &mut *self.0 };
        isolate.exit_isolate();
    }
}
