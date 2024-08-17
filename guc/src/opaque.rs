use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
#[derive(Debug)]
pub struct Opaque {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}
