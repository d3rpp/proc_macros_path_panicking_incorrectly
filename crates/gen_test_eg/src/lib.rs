#![feature(proc_macro_span)]

use proc_macro::{Span, TokenStream};

#[proc_macro]
pub fn gen_test_eg(_stream: TokenStream) -> TokenStream {
    // get the span for the call site so we can get files relative to that call site
    let invocation_span = Span::call_site();

    // get the call site file and directory locations
    let source_file = invocation_span.source_file();
    let path = source_file.path();
    let path_parent = path.parent();

    // both in the `rustc` instance and the `rust-analyzer`, this returns true.
    // by this logic, this is not a case of not reading a virtual file, but a bug
    let is_source_file_real = source_file.is_real();

    // this basic check could be used as a workaround for this issue by just
    // returning a blank TokenStream if it is the case to make rust-analyzer
    // happy
    let is_source_path_empty = path.as_os_str().is_empty();

    // panic with them so they show up in the editor
    panic!("path = {:?}, path_parent = {:?}, source_file_is_real = {is_source_file_real}, is_source_path_empty = {is_source_path_empty}", &path.display(), &path_parent.map(|item| item.to_string_lossy()));
}
