extern "C" {
    struct Spinner {
    }

    impl Spinner {
        pub fn setInterval(self, interval: cty::c_int);
        pub fn setText(self, text: *const cty::c_char);
        pub fn setSymbols(self, symbols: *const cty::c_char);
    }
}

fn spin() {
    unsafe {}
}
