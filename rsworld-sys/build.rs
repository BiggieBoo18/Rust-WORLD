fn main() {
    let world_dir:    &str = "World";
    let world_src:    &str = &format!("{}/{}", world_dir, "src");
    let world_header: &str = &format!("{}/{}", world_src, "world");

    let file_names = [
        "cheaptrick",
        "codec",
        "common",
        "d4c",
        "dio",
        "fft",
        "harvest",
        "matlabfunctions",
        "stonemask",
        "synthesis",
        "synthesisrealtime"
    ];
    for file_name in &file_names {
        cc::Build::new()
            .warnings(true)
            .flag("-O1")
            .flag("-Wall")
            .file(&format!("{}/{}.cpp", world_src, file_name))
            .include(world_header)
            .compile(&format!("{}.a", file_name));
    }
}

