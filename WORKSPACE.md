# Workspace Structure

This workspace contains two crates:

## `core`
The core library containing shared models and functionality.

Location: `/core`

## `dofus_db`
The Dofus DB library that depends on `core`. Contains database client, parser, and DB-specific models.

Location: `/dofus_db`

Dependencies:
- Depends on `core` crate

## Legacy Code
The `/src` directory contains legacy code that will be refactored and moved into the appropriate crates (`core` or `dofus_db`).

## Building

To build all crates in the workspace:
```bash
cargo build
```

To build a specific crate:
```bash
cargo build -p core
cargo build -p dofus_db
```

## Using `core` from `dofus_db`

The `dofus_db` crate already has `core` as a dependency. You can use it like:

```rust
use core::models::YourModel;
```

Or since core is re-exported:
```rust
use core::YourModel;
```

