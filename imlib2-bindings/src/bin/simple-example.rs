// This example is taken from Imlib2 documentation and rewritten as close as possible.
// https://docs.enlightenment.org/api/imlib2/html/
extern crate imlib2_bindings;

use std::ffi::CString;
use std::os::raw::*;
use imlib2_bindings::*;

fn to_cstring(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn main() {
    unsafe {
        let usage = "\nConvert image from one format to another.\n\n\
                          Usage:\n\
                          simple-example <source> <destination>\n";
        let mut args = std::env::args();
        let input_file = to_cstring(&args.nth(1).expect(usage));
        let output_file = args.next().expect(usage);
        let image = imlib_load_image(input_file.as_ptr() as *mut c_char);
        if !image.is_null() {
            imlib_context_set_image(image);
            if let Some(extension) = output_file.split('.').last() {
                imlib_image_set_format(to_cstring(&extension).as_ptr() as *mut c_char);
            }
            imlib_save_image(to_cstring(&output_file).as_ptr() as *mut c_char);
        }
    }
}