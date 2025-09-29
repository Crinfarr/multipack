mod curseforge;
mod modrinth;
pub mod curse {
    pub use super::curseforge::*;
}
pub mod mr {
    pub use super::modrinth::*;
}