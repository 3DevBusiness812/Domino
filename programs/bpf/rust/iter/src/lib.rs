//! Example Rust-based BPF program tests loop iteration

extern crate domino_program;
use domino_program::{custom_panic_default, entrypoint::SUCCESS, log::dom_log_64};

#[no_mangle]
pub extern "C" fn entrypoint(_input: *mut u8) -> u64 {
    const ITERS: usize = 100;
    let ones = [1_u64; ITERS];
    let mut sum: u64 = 0;

    for v in ones.iter() {
        sum += *v;
    }
    dom_log_64(0xff, 0, 0, 0, sum);
    assert_eq!(sum, ITERS as u64);

    SUCCESS
}

custom_panic_default!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_entrypoint() {
        assert_eq!(SUCCESS, entrypoint(std::ptr::null_mut()));
    }
}