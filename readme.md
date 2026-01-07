# Rust!

Based on Rust series posted at [Rust Jobs Blog](https://rustjobs.dev/blog/).

## Covered so far:

- [Kicking Off My Rust Journey](https://rustjobs.dev/blog/kicking-off-my-rust-journey/)
- [Episode 2 - Modeling Basic Data](https://rustjobs.dev/blog/episode-2-modeling-basic-data/)
- [Episode 3 - Fetching Data from an External API](https://rustjobs.dev/blog/episode-3-fetching-data-from-an-external-api/)
- [Episode 4 - Storing Data Locally](https://rustjobs.dev/blog/episode-4-storing-data-locally/)
- [Episode 5 - Scraping the Whole Dofus Encyclopedia](https://rustjobs.dev/blog/episode-5-scraping-the-whole-dofus-encyclopedia/)
- [Episode 6 - Testing in Rust — Because Even Pet Projects Deserve Some Love](https://rustjobs.dev/blog/episode-6-unit-testing-in-rust/)
- [Episode 7 - Completing the Model & Wiring a CLI](https://rustjobs.dev/blog/episode-7-completing-the-model-and-wiring-a-cli/)
- [Episode 8 - Overview of the Build Tool](https://rustjobs.dev/blog/episode-8-overview-of-the-build-tool/)
- [Episode 9 - Build Data Structure](https://rustjobs.dev/blog/episode-9-build-data-structure/)

## Next up:

- [Episode 10 - Searching Through Gear Combinations](https://rustjobs.dev/blog/episode-10-searching-through-gear-combinations/)
- [Episode 11 - Restricting the Dataset](https://rustjobs.dev/blog/episode-11-restricting-the-dataset/)

## Other links:

- [dofus-opti code](https://github.com/julien-truffaut/dofus-opti/tree/main)
- [What does "cannot move out of index" mean?](https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean)
- [Rust FizzBuzz](https://chrismorgan.info/blog/rust-fizzbuzz/)
- [Adding an element to an immutable vector](https://stackoverflow.com/questions/71165771/add-element-to-immutable-vector-rust)
- [Why does Rust have eq and partialeq?](https://www.reddit.com/r/rust/comments/t8d6wb/why_does_rust_have_eq_and_partialeq/)
- [Description of Rust module system](https://www.sheshbabu.com/posts/rust-module-system/)
- [Introduction to Cargo and cargo.toml](https://dev.to/alexmercedcoder/introduction-to-cargo-and-cargotoml-2l86)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- Following passage taken from Stack Overflow
  Question, [Why does rust need both packages and workspaces?](https://stackoverflow.com/questions/62051925/why-does-rust-need-both-packages-and-workspaces).  
  Source - https://stackoverflow.com/a  
  Posted by Danny Meyer  
  Retrieved 2026-01-05, License - CC BY-SA 4.0  

  `Crates:`
  They're the unit of compilation in Rust.
  A crate can be a binary (executable) or a library.
  Each crate has its own module hierarchy, and the top-most level is the crate root.

  `Packages:`
  A package contains one or more crates.
  While a package can contain multiple binary crates, it can contain at most one library crate. This is because Cargo,
  Rust's package manager, uses the package name as the library name when you're importing it as a dependency in another
  project. Having multiple libraries in a single package would be ambiguous.

  `Workspaces:`
  A workspace allows you to group multiple packages together.
  This is useful when you're working on multiple interdependent packages, and you want them to share the same Cargo.lock
  file and target directory.