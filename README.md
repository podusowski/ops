Ops is a tool for running predefined tasks (missions) in Docker.

## Installing

If you have Rust toolchain installed, you can simply use Cargo:

```
cargo install --git https://github.com/podusowski/ops.git
```

## Missions

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

## Shell

You can also enter an interactive shell, by defining it in `Ops.yaml`:

```yaml
shell:
  image: rust
```

## Mounting volumes

By default, Ops mounts the current directory in the container with the same
path.

It is also possible to mount an arbitrary volume using the `volumes` attribute.

```yaml
missions:
  create-a-file:
    image: busybox
    volumes:
      - /usr/local:/usr/local
    script: touch foo
```

## Forwarding user

Missions and shell will use Docker's default user, typically a `root` . This
affects the owner of files that are created in the workspace. To run containers
with as the current user, you can use `forward_user` in you missions and shell:

```yaml
missions:
  create-a-file:
    image: busybox
    forward_user: True
    script: touch foo
```

## Building the container image

Images can ether be downloaded by Docker, or built from the `Dockerfile`.

```yaml
shell:
  build: .
```

Above snippet will instruct Ops to build an image from context given in
`build` value.

For simple images, you can embed `Dockerfile` context directly in `Ops.yaml`:

```yaml
shell:
  recipe: |
    FROM rust
    RUN apt-get update && apt-get install -y docker.io
```
