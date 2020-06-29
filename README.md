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

## Get Started

Let's go through the basic usage with Sai. 

### Step 1: Define your component

Defining a component in Sai is as simple as defining a struct.
Annotate the struct with `#[derive(Component)]` will turn this struct to a component definition. 

```rust
use sai::{Component};

#[derive(Component)]
pub struct FooController {}

impl FooController {
    pub fn do_something (&self) {
        // Some logic
    }
}

#[derive(Component)]
pub struct DbPool {}

```

### Step 2: Declare dependencies using `#[injected]`

Components will naturally depend on each other to make things work. 
In above example, let's say `FooController` wants to access `DbPool`, 
all we need to do is to add `DbPool` as a field to `FooController` + annotate it using `#[injected]` + wrapped it with `Injected`.
The `System`, which we will go through later, will smartly prepare dependencies for you. 

```rust
use sai::{Component, Injected};

#[derive(Component)]
pub struct FooController {
    #[injected]
    pool: Injected<DbPool>
}

impl FooController {
    pub fn do_something (&self) {
        // self.pool is accessible here.
    }
}

// the rest is the same
```

You may wonder what happens to the ownership of `DbPool` component? 
In Sai, the `System` controls the lifecycle of all components. 
The `Injected` struct is basically a wrapper over `Arc`.

### Step 3 (Optional): Control the lifecycle of your component with `#[lifecycle]`

It's very common for a component have explict startup logic, 
e.g. initiate DB connection, bind port for web traffic, connect to message queue, etc. 

In Sai, to control the lifecycle of Component, simple annotate your component with `#[lifecycle]` and implement `ComponentLifecycle` for it.
```rust
use sai::{Component, Injected, ComponentLifecycle, async_trait};

// ... FooController is untouched

// Assuming Pool is a connection pool type
#[derive(Component)]
#[lifecycle]  // < --- NOTE HERE HERE 
pub struct DbPool {
    pool: Option<Pool>
}

#[async_trait]
impl ComponentLifecycle for DbPool {
    async start(&mut self) {
        println("Starting up DB connection pool...");
        // Just an example
        self.pool = Some(Pool::new(/*...*/))
    }
    async stop(&mut self) {
        println("Shutting down DB connection pool...");
        
        // You don't have to do much here: 
        // when System stops a Component, it will drop it as soon as possible.
        // But it's still good to ensure component shutdown cleanly instead of relying on Drop, 
        // though it's not always possible.
    }
}
```

Some notes:
- `async_trait` is necessary for implementing `ComponentLifecycle`. It's re-exported from [this library](https://github.com/dtolnay/async-trait).
- For fields that are not injected by Sai, they have to implement `Default` otherwise it won't compile. 

### Step 4: Create a System using components + kickstart the System

Once we have defined a few components, 
We just need to compose them into a System.
A System is a state machine that contains a collection of components. 
The collection of components is represented by `component registry` in Sai. 

```rust
use sai::{component_registry, System, /* other stuff... */};
use tokio::signal;

/* FooController + DbPool defined as above */

/* Define a component registry called RootRegistry which has two components */
component_registry(RootRegistry, [ FooController, DbPool ]);

#[tokio::main] // Or async-std
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // This is the key, we define a system using the RootRegistry.
    let mut system : System<RootRegistry> = System::new();
    
    println!("System starting up...");
    system.start().await;
    println!("System started.");

    // Waiting for Ctrl-c
    signal::ctrl_c().await?;

    println!("System shutting down...");
    system.stop().await;
    println!("System shutted down.");
    Ok(())
}

```

The System will in charge of lifecycles of all components. 
In `start`, the system will create and start all **registered** components one-by-one and wire them up according to their dependencies (see step 2).
In `stop`, the system will stop and **drop** all components in system one-by-one in the reverse order of `start`.

In large system, it's common to compose multiple registries into one, each registry can represent a module of the system. 
Sai provided a utility macro `combine_component_registry!` for it:

```rust
combine_component_registry!(RootRegistry, [ 
    ApiRegistry,
    WebRegistry,
    BusinessLogicRegistry,
    // Any number of registries
])
```

### 🎉🎉 You graduated! 
Thanks for going over this guide. 
Sai is a minimal library. 
Although this is called a "basic" guide, it already covers most of contents of this library. 
I hope Sai can help you.

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
