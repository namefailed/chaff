// Artifacts are bundled at compile time via include_str! in engine.rs.
// This module is kept as a hook for future runtime update support.

pub fn load_or_fetch() -> Option<String> {
    None
}
