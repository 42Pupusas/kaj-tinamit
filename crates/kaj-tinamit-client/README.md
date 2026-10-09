# kaj-tinamit-client

Blocking, type-safe GitLab REST API v4 client for Rust. It combines [`kaj-tinamit`](https://crates.io/crates/kaj-tinamit) wire models with [`kaj-tinamit-http`](https://crates.io/crates/kaj-tinamit-http) transport and exposes endpoint extension traits through `prelude`.

```toml
kaj-tinamit-client = "0.2"
```

```rust,no_run
use kaj_tinamit_client::{GitlabClient, prelude::*};

let client = GitlabClient::from_env()?;
let user = client.current_user()?;
println!("{}", user.username);
# Ok::<(), kaj_tinamit_client::Error>(())
```

Set `GITLAB_URL` and `GITLAB_PAT` to configure the client.

## License

Licensed under either of Apache License, Version 2.0 or MIT, at your option.

License: Apache-2.0 OR MIT

See the [workspace repository](https://github.com/42Pupusas/kaj-tinamit) for the full API documentation.
