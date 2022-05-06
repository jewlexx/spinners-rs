use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("cppspin/include/spinners.hpp")
        .compile("spinners")
}
