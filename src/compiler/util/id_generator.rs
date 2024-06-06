
static mut COUNTER: u64 = 0;

pub(crate) fn next_identifier() -> u64 {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
