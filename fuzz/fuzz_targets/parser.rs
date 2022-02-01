#![no_main]
use apollo_parser::Parser;
use apollo_rs_fuzz::generate_valid_document;
use libfuzzer_sys::fuzz_target;
use std::panic;

fuzz_target!(|data: &[u8]| {
    let doc_generated = match generate_valid_document(data) {
        Ok(d) => d,
        Err(err) => {
            // println!("error {err:?}");
            return;
        }
    };

    let parser = panic::catch_unwind(|| Parser::new(&doc_generated));

    let parser = match parser {
        Err(err) => {
            panic!("error {:?}", err);
        }
        Ok(p) => p,
    };

    println!("Parsing...");
    let tree = parser.parse();
    println!("Parsed");
    // early return if the parser detected an error
    let mut should_panic = false;
    for err in tree.errors() {
        should_panic = true;
        println!("Parser error ============= {err:?}");
    }
    if should_panic {
        panic!("error detected");
    }
});
