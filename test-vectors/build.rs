fn main() {
    // Rerun cargo when any test vectors (both real and testing) are modified
    println!("cargo::rerun-if-changed=src/vectors/");
    println!("cargo::rerun-if-changed=src/testdata/");
}
