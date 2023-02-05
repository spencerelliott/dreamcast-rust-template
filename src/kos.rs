pub struct Kos {}

#[repr(C)]
struct CFile
{
    _ptr: *mut char,
    _cnt: i32,
    _base: *mut char,
    _flag: i32,
    _file: i32,
    _charbuf: i32,
    _bufsiz: i32,
    _tmpfname: *mut char,
}

extern "C" {
    fn gdb_init();
    fn pvr_init_defaults();
    fn println(s: &str, ...);
}

impl Kos {
    pub fn test(self) -> () {
        unsafe {
            println("Hello, Rust!\n");
        }
    }
}