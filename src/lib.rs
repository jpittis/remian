#[macro_use]
extern crate helix;

ruby! {
    class Remian {
        def message() -> &'static str {
            return "foobar"
        }
    }
}
