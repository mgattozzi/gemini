# `gemini`
-----
An attribute proc macro that lets a user choose a sync or async API with no
overhead maintenance burden.

One of the frustrations I have is not getting to choose whether the code or APIs
I work with is sync or async. The same goes for writing a library as it should
be up to the caller to choose. The problem with this is that it makes it hard to
maintain with conditional compilation and the code structure is different
depending if it was async or not. Wouldn't it be nice to just write the code as
you normally would for an async function and then let the user choose which version of the
API they use? That's what `gemini` is made to do! Just slap on the attribute on
your functions and go about your day.

## Currently Unsupported
- sync functions that return `Box<dyn Future<Output = T>>` and `Pin<Box<dyn Future<Output = T>>>`
  - These are often needed to support recursion for async
- `async move` and `async` blocks
- Calls to `spawn` and `block_on` for various executors
- Probably more edge cases with futures and async

While these are unsupported they are things that should eventually be added. The
code as it is now only works with a very basic understanding of how async
functions are written in Rust.

## Basic Usage

In order to use `gemini` you only need to import it and put the attribute on
a function you wish to have be both sync and async like so:

```rust
use gemini::gemini;

#[gemini]
pub async fn basic() -> Result<(), String> {
  func().await?;
  Ok(())
}

#[gemini]
pub async fn func() -> Result<(), String> {
  Ok(())
}
```

This will then let your user choose with conditional compilation flags which
version of your code they wish to use. In the case of async code it expands out
to:

```rust
use gemini::gemini;

#[cfg(not(feature = "sync"))]
pub async fn basic(item: Vec<String>) -> Result<(), String> {
    func().await?;
    Ok(())
}

#[cfg(not(feature = "sync"))]
pub async fn func() -> Result<(), String> {
    Ok(())
}
```

As for the sync version it looks like:

```rust
use gemini::gemini;

#[cfg(feature = "sync")]
pub fn basic() -> Result<(), String> {
    func()?;
    Ok(())
}

#[cfg(feature = "sync")]
pub fn func() -> Result<(), String> {
    Ok(())
}
```

In your `Cargo.toml` add this section:

```toml
[features]
default = []
sync = []
```

With this your users can use the default async API by listing your crate in
their `Cargo.toml` file with:

```toml
[dependencies]
your-crate = "<version number here>"
```

If they want to use the sync API then their `Cargo.toml` should look like this:

```toml
[dependencies]
your-crate = { version = "<version number here>", features = ["sync"] }
```

It's that simple. `gemini` takes care of all the maintenance burden and you get
to let your api work in both contexts and giving your users more power to choose
what they want.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
