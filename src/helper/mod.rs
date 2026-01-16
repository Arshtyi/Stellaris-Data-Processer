pub mod exit;
pub mod path;

pub use exit::safely_exit;
pub use path::{resolve_path, resolve_path_and_create};
