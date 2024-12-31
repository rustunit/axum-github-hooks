# axum-github-hooks

[![crates.io][sh_crates]][lk_crates]
[![docs.rs][sh_docs]][lk_docs]
[![discord][sh_discord]][lk_discord]

[sh_crates]: https://img.shields.io/crates/v/axum-github-hooks.svg
[lk_crates]: https://crates.io/crates/axum-github-hooks
[sh_docs]: https://img.shields.io/docsrs/axum-github-hooks
[lk_docs]: https://docs.rs/axum-github-hooks/latest/axum-github-hooks/
[sh_discord]: https://img.shields.io/discord/1176858176897953872?label=discord&color=5561E6
[lk_discord]: https://discord.gg/rQNeEnMhus

Axum extractor for GitHub webhooks.

# Usage

```rust
let router = Router::new().route("/", post(webhook));

async fn webhook(GithubWebhook(hook): GithubWebhook) -> impl IntoResponse {
    // TODO:

    StatusCode::OK
}
```

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
