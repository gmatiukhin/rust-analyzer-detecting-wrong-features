pub const HELLO_UNCONDITIONAL: &str = "You can read this unconditionally";

#[cfg(feature = "feat")]
pub mod feat {
    pub const HELLO_CONDITIONAL: &str = "You can read this only if you use feature 'feat'";
}
