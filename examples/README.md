# Examples

## basic

In this example, tokio is chosen as the runtime.
The system contains four components:

- A [Tide](https://github.com/http-rs/tide) server component
- A [Gotham Server](https://gotham.rs/) component
- DB pool
- A simple tide controller

[mockall](https://github.com/asomers/mockall) is used to demonstrate how to unit test a single component, see `foo_controller.rs`.

### To Run

Start a testing DB container: `docker run -it --rm -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres:10-alpinee`

Run example: `cargo run -p examples --example basic`

You'll see:

```
System starting up...
Starting DB connection...
Starting TideServer...
Starting Gotham Server...
System started.
```

And then press Ctrl-C, you will see:

```
^CSystem shutting down...
Shutting Gotham server...
Gotham server gracefully shutted down...
Shutting down TideServer...
TideServer gracefully shutted down...
Shutting down DB connections...
System shutted down.
```
