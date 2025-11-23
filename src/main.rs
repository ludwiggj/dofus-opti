// The main binary.

// The second step is to compile the binary. The compiler will take main.rs and other 
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}