fn main() {
    // cargo api key can be found at crates.io
    // saved via: cargo login [key]

    // Crate requirements:
    // - Unique name (stored in toml as package->name)
    // - Description (stored in toml as package->description)
    // - License
    //      stored in toml as package->license
    //      options listed in Linux Foundation SPDX list, e.g. MIT
    //      multiple licenses may be separated by OR, e.g. MIT OR Apache-2.0
    // - There needs to actually be at least one line of code :p

    // To publish, run: cargo publish
    // To publish a new version, incremement version and run: cargo publish

    // Remote specific version:
    //   cargo yank --vers 1.0.1
    // To undo a yank
    //   cargo yank --vers 1.0.1 --undo

    println!("Some code just in case that makes things happy");
}
