// When the crate is compiled, the first step is to compile the library.

// The following modules are part of the library
pub mod dofus_db_import;
pub mod dofus_db_export;
pub mod dofus_db_file;

// Re-export models from core crate
pub mod models {
    pub use core::*;
}

// Re-export dofus_db_models from dofus_db crate
pub mod dofus_db_models {
    pub use dofus_db::model::*;
}

// Re-export dofus_db_client from dofus_db crate
pub mod dofus_db_client {
    pub use dofus_db::client::*;
}