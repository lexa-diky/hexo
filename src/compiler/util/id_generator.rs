use std::collections::HashMap;

static mut COUNTER: u64 = 0;

pub(crate) fn next_identifier() -> u64 {
    unsafe {
        COUNTER = COUNTER + 1;
        COUNTER
    }
}
