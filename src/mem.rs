use core::ffi::c_void;

#[no_mangle] // Export with the exact name "memset" to be recognized by the linker
pub unsafe extern "C" fn memset(ptr: *mut c_void, value: i32, num: usize) -> *mut c_void {
    let byte_ptr = ptr as *mut u8; // Cast the void pointer to a byte pointer
    for i in 0..num {
        *byte_ptr.add(i) = value as u8;
    }
    ptr
}
