fn main() {
    let target = std::env::var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    let is_msvc = match target.get(3) {
        Some(&"msvc") => true,
        _ => false,
    };

    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("windivert.h");

    // Include the directory where the header files are defined
    cfg.include(concat!(env!("CARGO_MANIFEST_DIR"), "/../vendored/divert/include"))
        .include(concat!(env!("CARGO_MANIFEST_DIR"), "/../vendored/divert/dll"));

    // Structs are typedefed in this library but the default type_name function will prepend a "struct", so replace it
    // with the bare name instead.
    cfg.type_name(|ty, _, _| {
        ty.to_string()
    });

    // Skip some fields because they involve bitfields and it's not a 1-1 translation
    cfg.skip_field(|_, field_name| {
        field_name.contains("bitfield")
    });

    cfg.fn_cname(|rust, _| {
        format!("{}", rust)
    });

    if is_msvc {
        // Library uses nonstandard bitfields, disable warning C4214
        cfg.flag("/wd4214");
    };

    // Our enums values are always positive, so don't worry about signedness
    cfg.skip_signededness(|ty_name| {
        ty_name == "WINDIVERT_LAYER" || ty_name == "WINDIVERT_PARAM"
    });

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}