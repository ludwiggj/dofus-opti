// When the crate is compiled, the first step is to compile the library.

// The following modules are part of the library
pub mod dofus_db_client;
pub mod dofus_db_file;
pub mod dofus_db_models;
pub mod dofus_db_parser;
pub mod models;
pub mod dofus_db_import;

// Code to share across binaries
pub mod superceded;