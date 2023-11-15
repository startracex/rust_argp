# rust_argp

Arguments-likes strings parser

From arguments

```rust
let mut ap = rust_argp::new();
```

From `Vec<String>`

```rust
let origin :Vec<String> =...;
let mut ap = rust_argp::from(origin);
```

Query boolean

```rust
let exist = ap.bool(&["...","..."]);
```

```rust
let mut exist = false;
ap.bool_var(&mut exist, &["...","..."]);
```

Query string

```rust
let (value, exist) = ap.string(&["...", "..."]);
```

```rust
let mut value = String::new();
ap.string_var(&mut value, &["...", "..."]);
```