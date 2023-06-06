fn main() {
    println!("cargo:rustc-link-lib=pfapack");
    println!("cargo:rustc-link-lib=cpfapack");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=blas");
}
