<h1 align="center">Sai</h1>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/sai">
    <img src="https://img.shields.io/crates/v/sai.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/sai">
    <img src="https://img.shields.io/crates/d/sai.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/sai">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/sai">
      API Docs
    </a>
    <span> | </span>
    <a href="examples">
      Examples
    </a>
  </h3>
</div>

Sai is a framework for managing lifecycle and dependency of your software components.
In some languages, it was called "IoC" and "Dependency Injection".
The main usecase of this framework is on medium/large scale web services.

The Sai ecosystem consists of two major concepts: [System](struct.System.html), [Component](trait.Component.html).
A System is a runtime unit that control lifecycles of all Components.
A Component is a group of logic. A Component can depends on other Components and it can
also have its own internal state.

## Features
- ✅ Build for Async Rust
- ✅ Minimal boilerplate
- ✅ Runs in stable

## FAQs

- Q: What does "Sai" mean?
  - Nothing really. It happens to be my cat's name. I can't find a good enough name because cargo has only single namespace and many good names are reserved (yes, they are reserved rather than used).
  
- Q: Why do I need this library?
  - It's tedius and error prone to pass common dependencies via multple layers of functions.
  - In a medium/large sized web service, it's important to have a granular control over startup / shutdown logic. Without a good framework, it's difficult to do things like: 
    - Get all secrets from secret manager
    - Then start DB/Redis connection
    - Then start listening x port for traffic
    - Then start a new server for healch check
    - In the end, shutdown all above in the reverse order

- Q: Does this handle circular dependency?
  - No, it does not currently.
  
- Q: Is any there limitation? 
  - Currently, it's hard to find Async Rust libraries that has a perfect/granular control over shutdown. 
  - Error handling/reporting in this library isn't perfect. (WIP)
  - Can't handle circular component dependency at this moment. (PR welcomed)

## Related projects

- [Component](https://github.com/stuartsierra/component) (Clojure)
- [InversifyJS](https://github.com/inversify/InversifyJS) (Javascript/Typescript)
