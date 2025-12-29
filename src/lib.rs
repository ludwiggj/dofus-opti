// When the crate is compiled, the first step is to compile the library.

// The following modules are part of the library
// Public modules can be used in this package's binary and by other rust code that imports this library
// Non-public modules can only be used within this library
pub mod dofus_db_client;
pub mod dofus_db_file;
pub mod dofus_db_models;
pub mod dofus_db_parser;
pub mod models;
pub mod dofus_db_import;
pub mod dofus_db_export;

// Code to share across binaries
pub mod superceded;