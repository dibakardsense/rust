//@revisions: extern_block definition both
//@normalize-stderr-test: "unsafe \{ libc::abort\(\) \}|crate::intrinsics::abort\(\);" -> "ABORT();"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
//@normalize-stderr-test: "\n  +[0-9]+:[^\n]+" -> "$1"
//@normalize-stderr-test: "\n at [^\n]+" -> "$1"
//@[definition,both]error-in-other-file: aborted execution
#![feature(rustc_attrs, c_unwind)]

#[cfg_attr(any(definition, both), rustc_nounwind)]
#[no_mangle]
extern "C-unwind" fn nounwind() {
    panic!();
}

fn main() {
    extern "C-unwind" {
        #[cfg_attr(any(extern_block, both), rustc_nounwind)]
        fn nounwind();
    }
    unsafe { nounwind() }
    //~[extern_block]^ ERROR: unwinding past a stack frame that does not allow unwinding
}
