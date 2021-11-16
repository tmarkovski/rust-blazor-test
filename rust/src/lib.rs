use std::{borrow::Borrow, mem, ptr, slice};

use ffi_support::ByteBuffer;

#[no_mangle]
pub extern "C" fn int_in_return(x: i32) -> i32 {
    // uncommenting below line throws, even if
    // the variable or data is not used at all
    let _data = vec![0, 1, 2];
    x
}

#[no_mangle]
pub extern "C" fn int_in_out(x: i32, y: &mut i32) -> i32 {
    *y = x;
    0
}

#[no_mangle]
pub extern "C" fn array_in(arr_len: i32, array_data: *const u8) -> i32 {
    let input = unsafe { slice::from_raw_parts(array_data, arr_len as usize) };

    input.len() as i32
}

#[no_mangle]
pub extern "C" fn array_in_out(arr_len: i32, array_data: *const u8, out_len: &mut i32, out_arr: &mut *mut u8) {
    let input = unsafe { slice::from_raw_parts(array_data, arr_len as usize) };

    let mut data = [1, 2, 3, 4, 5];

    // this work, when invoking twice
    // first time it produces [0, 0, 0, 0, 0], then returns fine
    *out_len = data.len() as i32;
    *out_arr = data.as_mut_ptr();
}

#[no_mangle]
pub extern "C" fn array_in_return(arr_len: i32, array_data: *const u8) -> *mut u8 {
    let input = unsafe { slice::from_raw_parts(array_data, arr_len as usize) }.to_vec();

    let mut buf = input.into_boxed_slice();
    let data = buf.as_mut_ptr();
    // std::mem::forget(buf);
    Box::leak(buf);

    // *out_len = arr_len;

    data
}

#[no_mangle]
pub extern "C" fn buffer_in(buffer: &ByteBuffer) -> i32 {
    buffer.as_slice().len() as i32
}

#[no_mangle]
pub extern "C" fn buffer_in_return(buffer: &ByteBuffer) -> ByteBuffer {
    ByteBuffer::from_vec(buffer.as_slice().to_vec())
}

// This is identical to ffi_support::ByteBuffer
// with fields made public to be assigned manually
#[repr(C)]
pub struct BBuffer {
    pub len: i64,
    pub data: *mut u8,
}


#[no_mangle]
pub extern "C" fn buffer_in_out(buffer: ByteBuffer, buffer_out: &mut ByteBuffer) {

    // uncommenting the below line will make this function throw
    let mut data = buffer.as_slice().to_vec();
    data.reverse();
    let res = ByteBuffer::from_vec(data);

    // below also throws
    // "Unhandled Promise Rejection: Error: JS object instance with ID NaN does not exist (has it been disposed?)."
    // "TypeError: Buffer is already detached"
    let mut result = vec![4u8, 5, 6].into_boxed_slice();
    let len = result.len();
    let result_ptr = result.as_mut_ptr();

    mem::forget(result);

    *buffer_out = res //BBuffer { len: len as i64, data: result_ptr }

    // below line completely hangs the browser
    // let mut data = vec![1, 2, 3, 4, 5];

    // let mut data = [1, 2, 3, 4, 5];
    // *buffer_out = BBuffer { len: 0, data: data.as_mut_ptr() }
}

