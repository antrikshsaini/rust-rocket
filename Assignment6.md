# Rust Web Server Project

> Using the compiler to prove the correctness of a program

## Background

The creation of a simple web server is a relatavely simple task. A web server receives a `Request` from a client over a network and returns a `Response` to the client. In the node ecosystem, this is typically done with the `express` or `http` modules. These packages can create a robust web solution with relatevly little work. The same thing can be achieved in rust!

Through our class learnings we have evaluated rust as a strongly typed, expressive, and exhaustive solution for building reliable apps. This makes rust a great solution for a reliable and deterministic web implementation.

## Description

Using rust, create a web server with the following features:

- routing
- validation
- authentication
- authorization (guards)
- dependency inversion
- type level proof
  - `authenticate`: `Cookie` -> `Optional<Authorized>` // The only way to get `Optional<Authorized>` is via the `authenticate` method
  - `f`: `Authorized` // Must have called the authenticate method
  - Example: https://github.com/agmoss/oranger/blob/main/src/main.rs#L127

## Possible Starters

> Is this color orange?!

```bash
# Determine if rgb is orange
curl --header "Content-Type: application/json" \
 --request POST \
 --data '{"r":255,"g":165,"b":0}' \
 http://localhost:8000/orange
```

## Useful links

[rocket](https://rocket.rs/v0.4/)
[comparison-of-rust-web-frameworks](https://github.com/flosse/rust-web-framework-comparison#server-frameworks)

## Extensions

- What elements of FRP can be employed in this app?
- What modules from `crates.io` can be used in this app?
- Use pattern matching, function composition, and other class learnings in this app
- In addition to API routes, create a UI route(s) for human interaction
