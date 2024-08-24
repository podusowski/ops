Ops is a tool for running predefined tasks (missions) in Docker.

Missions are defined in file called `Ops.yaml` and the behave more or less like
jobs on CI systems. In fact, Ops aims to streamline the CI and local development.

A mission can look like this:

```yaml
missions:
  check-rust:
    image: rust
    script: cargo check
```

It will run Docker using `rust` image, mounting current directory with the same
name, and passing `script` content to the default command (typically a shell).

You can also enter an interactive shell, by defining it in `Ops.yaml`:

```yaml
shell:
  image: rust
```
