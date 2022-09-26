extern "C" {
    fn cool_function(a: isize, b: isize) -> isize;
}

fn main() {
    unsafe { cool_function(2, 9); }
}
