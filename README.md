Ops is a tool for running predefined tasks (missions) in Docker. Simple mission can look like this:

```
missions:
  check-rust:
    image: rust
    script: cargo check
```

# The Plan

Following functionalities are planned.

## Local mode

- `run` - Run the given task/recipe on a local working copy.

## CI mode

- `serve` - Watch a remote (or local) git repository and act as a CI server.
