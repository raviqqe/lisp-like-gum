use libc::{c_void, malloc, free};



pub fn alloc_memory(s: usize) -> usize {
  let p = unsafe { malloc(s) as usize };

  if p == 0 {
    panic!("libc::malloc() failed to allocate memory.")
  }

  p
}

pub fn free_memory(p: usize) {
    unsafe { free(p as *mut c_void) }
}
