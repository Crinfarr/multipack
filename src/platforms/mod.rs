mod curseforge;
mod modrinth;
pub mod mod_data;
pub mod curse {
    pub use super::curseforge::*;
    pub use super::mod_data::curse_mod_data::*;
}
pub mod mr {
    pub use super::modrinth::*;
    pub use super::mod_data::modrinth_mod_data::*;
}