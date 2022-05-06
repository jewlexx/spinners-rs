extern "C" {
    struct Spinner {
    }

    impl Spinner {
        pub fn setInterval(self, interval: cty::c_int);
        pub fn setText(self, text: *const cty::c_char);
        pub fn setSymbols(self, symbols: *const cty::c_char);

        pub fn startSpinner(self);
        pub fn start(self);
        pub fn stop(self);
    
    }
}

fn spin() {
    unsafe {}
}
