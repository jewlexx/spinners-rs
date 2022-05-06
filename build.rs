use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("cppspin/include/spinners.hpp")
        .flag_if_supported("-Werror")
        .flag_if_supported("-Wall")
        .flag_if_supported("-pedantic")
        .compile("spinners.a")
}
