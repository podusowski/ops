Ops is a tool for running predefined tasks (missions) in Docker.

Missions are defined in file called `Ops.yaml` and can look like this:

```yaml
missions:
  check-rust:
    image: rust
    script: cargo check
```
