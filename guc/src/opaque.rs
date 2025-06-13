#[repr(C)]
#[derive(Debug)]
pub struct Opaque {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

use core::marker::PhantomData;
use core::marker::PhantomPinned;
