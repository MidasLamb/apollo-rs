use apollo_smith::DocumentBuilder;
use libfuzzer_sys::arbitrary::{Result, Unstructured};

pub fn generate_valid_document(
    input: &[u8],
    // configure: impl FnOnce(&mut SwarmConfig, &mut Unstructured<'_>) -> Result<()>,
) -> Result<String> {
    let mut u = Unstructured::new(input);
    let gql_doc = DocumentBuilder::new(&mut u)?;
    let document = gql_doc.finish();
    // Use wasm-smith to generate an arbitrary module and convert it to wasm
    // bytes.
    // let module = document::document(&mut u);
    // let module_str = module.finish();
    println!(
        "doc =====================\n {}\n========================",
        document
    );

    Ok(document)
}

// // Optionally log the module and its configuration if we've gotten this
// // far. Note that we don't do this unconditionally to avoid slowing down
// // fuzzing, but this is expected to be enabled when debugging a failing
// // fuzzer.
// pub fn log_wasm(wasm: &[u8]) {
//     drop(env_logger::try_init());

//     if log::log_enabled!(log::Level::Debug) {
//         log::debug!("writing test case to `test.wasm` ...");
//         std::fs::write("test.wasm", wasm).unwrap();
//         std::fs::write("test.config", format!("{:#?}", config)).unwrap();
//         if let Ok(wat) = wasmprinter::print_bytes(wasm) {
//             log::debug!("writing text format to `test.wat` ...");
//             std::fs::write("test.wat", wat).unwrap();
//         } else {
//             drop(std::fs::remove_file("test.wat"));
//         }
//     }
// }
