Terminal Countdown Timer (Rust implementation)
===

The original implementation of this was inspired by zenito9970  
Original implementation => [zenito9970/countdown](https://github.com/zenito9970/countdown-rs)

Usage
---

Specify duration in go format `1h2m3s` .

```
countdown-rs 25s TAREA
```

Press `Esc` or `Ctrl+C` to stop countdown without running next command.

Install
---

```
git clone https://github.com/holoju/countdown-rs.git
cd countdown-rs
cargo install --path .
```

License
---

MIT
