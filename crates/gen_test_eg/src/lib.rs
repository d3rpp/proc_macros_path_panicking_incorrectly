#![feature(proc_macro_span)]

use proc_macro::{Span, TokenStream};


#[proc_macro]
pub fn gen_test_eg(_stream: TokenStream) -> TokenStream {
    // get the span for the call site so we can get files relative to that call site
    let invocation_span = Span::call_site();

    // get the call site file and directory locations
    let path = invocation_span.source_file().path();
    let path_parent = path.parent();

    // panic with them so they show up in the editor
    panic!("path = {:?}, path_parent = {:?}", &path.display(), &path_parent.map(|item| item.to_string_lossy()));
}