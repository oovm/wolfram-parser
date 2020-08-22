use wolfram_ast::ast::WolframInputs;
use wolfram_error::{FileCache, Result};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_vm() -> Result<()> {
    let mut store = FileCache::default();
    let input1 = store.load_text("5*y", "input1");
    let input2 = store.load_text("5y", "input2");
    let input3 = store.load_text("x^5*y", "input3");
    let input4 = store.load_text("x^5y", "input4");
    match WolframInputs::from_cache(input1, &store) {
        Ok(o) => {
            println!("Input1: {o:#?}")
        }
        Err(e) => e.as_report().eprint(&store)?,
    };
    match WolframInputs::from_cache(input2, &store) {
        Ok(o) => {
            println!("Input2: {o:#?}")
        }
        Err(e) => e.as_report().eprint(&store)?,
    };
    match WolframInputs::from_cache(input3, &store) {
        Ok(o) => {
            println!("Input3: {o:#?}")
        }
        Err(e) => e.as_report().eprint(&store)?,
    };
    match WolframInputs::from_cache(input4, &store) {
        Ok(o) => {
            println!("Input4: {o:#?}")
        }
        Err(e) => e.as_report().eprint(&store)?,
    };
    Ok(())
}
