use core::ffi::c_void;

#[no_mangle] // Export with the exact name "memset" to be recognized by the linker
pub unsafe extern "C" fn memset(ptr: *mut c_void, value: i32, num: usize) -> *mut c_void {
    let byte_ptr = ptr as *mut u8; // Cast the void pointer to a byte pointer
    for i in 0..num {
        *byte_ptr.add(i) = value as u8;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let dest_slice = unsafe { core::slice::from_raw_parts_mut(dest, n) };
    let src_slice = unsafe { core::slice::from_raw_parts(src, n) };
    dest_slice.copy_from_slice(src_slice);
    dest
}


#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let s1_slice = unsafe { core::slice::from_raw_parts(s1, n) };
    let s2_slice = unsafe { core::slice::from_raw_parts(s2, n) };
    for (a, b) in s1_slice.iter().zip(s2_slice.iter()) {
        if a != b {
            return (*a as i32) - (*b as i32);
        }
    }
    0
}