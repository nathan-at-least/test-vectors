//! Execute tests against test vectors stored in external files
//!
//! The [macro@test_vectors] macro annotates a _test criterion function_ which executes against
//! multiple _cases_. Each case expands to a standalone rust unit test (ie a `#[test]` function). The
//! data for each case is stored in a _case directory_, where each argument to the test criterion
//! function is associated with a file. All of the case directories live inside a _corpus
//! directory_ which is specified with the `dir` parameter to [macro@test_vectors].
//!
//! # Example
//!
//! Suppose we have a fancy-dancy crate which can replace spaces with hyphens in a string, and we
//! want to test that functionality against a bunch of input vector files.
//!
//! We can organize the crate contents like this:
//!
//! - `Cargo.toml` - depending on [test-vectors](crate) in `[dev-dependencies]`
//! - `src/lib.rs` - containing the example code below
//! - `test-data/example1/alpha/input` - containing `this is alpha`
//! - `test-data/example1/alpha/expected` - containing `this_is_alpha`
//! - `test-data/example1/beta/input` - containing `this is beta`
//! - `test-data/example1/beta/expected` - containing `this_is_beta`
//!
//! Now in our `lib.rs` we have:
//!
//! ```
//! pub fn replace_spaces_with_underscores(input: &str) -> String {
//!     input.replace(' ', "_")
//! }
//!
//! #[test_vectors::test_vectors(
//! # doctest = true,
//!   dir = "test-data/example1"
//! )]
//! fn test_replace(input: &[u8], expected: &[u8]) -> Result<(), std::str::Utf8Error> {
//!     // Test setup:
//!     let instr = std::str::from_utf8(input)?;
//!     let expstr = std::str::from_utf8(expected)?;
//!
//!     // Application code test target:
//!     let output = replace_spaces_with_underscores(instr);
//!
//!     // Test verification:
//!     assert_eq!(expstr, &output);
//!
//!     Ok(())
//! }
//! ```
//!
//! This creates two rust unit tests from the case directories inside the corpus directory
//! `test-data/example1`. The cases are named after the case directories `alpha` and
//! `beta`. For each test, the file contents of the `input` and `expected` files in the case
//! directory are mapped to the `&[u8]` test criterion function arguments. The output of `cargo
//! test` will include something like this:
//!
//! ```text
//! test test_replace_alpha ... ok
//! test test_replace_beta ... ok
//! ```
//!
//! # Motivations
//!
//! This design is well suited to tests which benefit from any of these features:
//!
//! - Separate the same criterion function into separate cases by input (similar to the
//!   [test-case](https://docs.rs/test-case) crate). If a subset of cases fail for the same test
//!   criterion, the `cargo test` output immediately identifies the specific failing cases, in
//!   contrast to a single `#[test]` function that loops over test vectors as a whole.
//! - Test raw un-encoded data stored directly in files, rather than rust-specific literal
//!   representations. This can help avoid divergence between live production data versus rust
//!   literal representations aimed at representing the data.
//! - Test against external files, which facilitates _conformance testing_
//!   of multiple implementations against a common set of test vectors. For example, a network
//!   protocol standard may include a set of message serialization test
//!   vectors which multiple implementations validate against.
//! - Use other external tools on the external data files. For example,
//!   if a video codec metadata parsing library has external test vector files, other tools for
//!   examining that video format, such as interactive video players, can be used directly
//!   on the test vectors.
//!
//! # Corpus and Case Directories
//!
//! The corpus directory is specified by the `dir` macro argument. This is a path relative to the
//! `CARGO_MANIFEST_DIR` environment variable (which is where the crates `Cargo.toml` lives).
//!
//! Every directory inside a corpus directory is expected to be a case directory (after
//! traversing symlinks). Non-directories are ignored, and it's good practice to have a `README.md`
//! file explaining the corpus.
//!
//! Inside a case directory, only the paths derived from the criterion function argument names are
//! accessed, and other contents are ignored, so a good practice is a `README.md` explaining
//! the intention of the case. Another nuance of this behavior is that different criterion
//! functions might reuse the same corpus directory.
//!
//! For example, a case directory under `test-data/example2` might have these files:
//!
//! - `input` with content `this is the input`
//! - `underscores` with the content `this_is_the_input`
//! - `elided` with the content `thisistheinput`
//!
//! Then two different criterion functions might test different conversions of the same inputs:
//!
//! ```
//! use test_vectors::test_vectors;
//! use std::str::Utf8Error;
//!
//! #[test_vectors(
//! # doctest = true,
//!   dir = "test-data/example2"
//! )]
//! fn replace_spaces_with_underscores(input: &[u8], underscores: &[u8]) -> Result<(), Utf8Error> {
//!     let instr = std::str::from_utf8(input)?;
//!     let expstr = std::str::from_utf8(underscores)?;
//!     let output = instr.replace(' ', "_");
//!     assert_eq!(expstr, &output);
//!     Ok(())
//! }
//!
//! #[test_vectors(
//! # doctest = true,
//!   dir = "test-data/example2"
//! )]
//! fn elide_spaces(input: &[u8], elided: &[u8]) -> Result<(), Utf8Error> {
//!     let instr = std::str::from_utf8(input)?;
//!     let expstr = std::str::from_utf8(elided)?;
//!     let output = instr.replace(' ', "");
//!     assert_eq!(expstr, &output);
//!     Ok(())
//! }
//! ```
//!
//! Since both criterion functions use the same corpus and both take `input`, they are testing
//! against the same test vector `input` files, while each function reads a different test vector
//! for its specific functionality, ie `underscores` vs `elided`.
//!
//! # Automatic Input Conversion From Bytes
//!
//! Arguments to criterion test functions are translated with `TryFrom<&[u8]>` against the
//! file contents, which are available as `&[u8]`. Since this trait provides a blanket
//! implementation, an argument of type `&[u8]` is the basic supported type.
//!
//! For other types, this can take care of some boiler-plate for converting inputs by using a
//! standard rust trait. This approach, versus supporting customizeable conversions in the macro
//! interface keeps the macro interface and logic simpler by relying on this standard rust trait.
//!
//! The result of conversion is unwrapped, so any failure of conversion causes a panic and the test
//! case will fail. The call site looks something like:
//!
//! ```text
//! <T>::try_from(include_bytes!(???)).unwrap()
//! ```
//!
//! Recall in the first example, we explicitly called [std::str::from_utf8] to convert the byte
//! slice parameters. This is an example of a conversion function that is not available via
//! `TryFrom<&[u8]>` (because there might be multiple ways to convert bytes into a `str`). So that
//! example highlights how test criterion functions may need to rely on newtype wrapper types to
//! perform conversions. The [test-vectors](crate) crate provides some commonly needed wrapper types, such as [Utf8Str] for that case. Compare the example in the [Utf8Str] docs to the first example above.
//!
//! If a test needs some custom conversion, it may need to implement a custom new-type wrapper, as
//! the next example shows:
//!
//! # Example of Implementing a Custom Conversion New-Type
//!
//! Suppose your crate type `T` implements [serde](https://doc.rs/serde)'s `Deserialize`
//! trait, your test vectors are JSON data, and you want to remove the boilerplate of deserializing
//! JSON in your criterion functions.
//!
//! You could implement a newtype that performs the conversion for you:
//!
//! ```
//! use serde::Deserialize;
//! use test_vectors::test_vectors;
//!
//! #[derive(Deserialize)]
//! struct AppType {
//!     valid: bool
//! }
//!
//! impl AppType {
//!     fn is_valid(&self) -> bool {
//!         self.valid
//!     }
//! }
//!
//! struct AppTypeFromJson(AppType);
//!
//! impl std::ops::Deref for AppTypeFromJson {
//!     type Target = AppType;
//!
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! impl TryFrom<&[u8]> for AppTypeFromJson
//! {
//!     type Error = serde_json::Error;
//!
//!     fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
//!         serde_json::from_slice(input).map(AppTypeFromJson)
//!     }
//! }
//!
//! #[test_vectors(
//! # doctest = true,
//!   dir = "test-data/example3"
//! )]
//! fn validate(value: AppTypeFromJson) {
//!     // Perform test-logic on `value`:
//!     assert!(value.is_valid());
//! }
//! ```
//!
//! # Criterion Function Return Type
//!
//! The return type of a criterion function is replicated directly for each test case, and the test
//! returns the criterion function result unaltered. Criterion functions can return `()` or [Result] with identical behavior to unit tests.

mod utf8str;

pub use self::utf8str::Utf8Str;
pub use test_vectors_macro::test_vectors;
