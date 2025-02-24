# Shrink

A URL Shortener built for [blazinglyfast.net](https://blazinglyfast.net/).

## Design

![Architecture](./architecture.png)

## Starting (URL Shortener) Shrink Server

### With Docker Compose

```console
docker compose up
```

### Locally

#### Launch Redis Server

> NOTE: The server is configured to use Redis for caching, although the program
> is customizable to opt out of caching and not depend on Redis.
> Please install `redis-server` if not present.

```console
redis-server
```

#### Run Rust Server Binary

```console
cargo run --release
```

## Shortening URLs

Bash Scripts under `./scripts` can be used to interact with the running server.

```bash
./scripts/shrink.sh https://blazinglyfast.net/
# {"shrunk":"http://localhost:3000/hWU7Xgc"}
```

### Custom Alias for a URL

```bash
./scripts/alias.sh blaze https://blazinglyfast.net/
# {"shrunk":"http://localhost:3000/blaze"}
```
