//! # Shine
//!
//! Shine is a framework for managing lifecycle and dependency of your software components.
//! In some languages, it was called "IoC" and "Dependency Injection".
//! The main usecase of this framework is on medium/large scale web services.
//!
//! The Shine ecosystem consists of two major concepts: [System](struct.System.html), [Component](trait.Component.html).
//! A System is a runtime unit that control lifecycles of all Components.
//! A Component is a group of logic. A Component can depends on other Components and it can
//! also have its own internal state.

use std::boxed::Box;
use std::any::{TypeId};

/// Re-export from async_trait library
pub use async_trait::async_trait;

pub use component_derive::Component;

mod injected;
pub use injected::Injected;

mod component_repository;
#[doc(hidden)]
pub use component_repository::ComponentRepository;

mod system;
pub use system::System;

mod downcast;

mod registry;
#[doc(inline)]
pub use registry::ComponentRegistry;

/// ComponentLifecycle is simply start()/stop()
///
/// You will have to manually implement this trait for your component if you want to have explict
/// startup/shutdown logic.
///
/// Check out the doc for [Component](trait.Component.html) trait
#[async_trait()]
pub trait ComponentLifecycle: Send { // Extend Send compiler stop complaining trait object issue
    async fn start(&mut self) {}
    async fn stop(&mut self) {}
}

/// A Component is a bunch of business-logic behaviors + startup logic.
///
/// Normally, you won't need to manually implement this.
///
/// **Defining components with no explict startup / shutdown logic**
/// ```
/// use shine::{Component, Injected};
/// #[derive(Component)]
/// struct Bar {}
///
/// #[derive(Component)]
/// struct Foo {
///     #[injected]
///     bar: Injected<Bar>
/// }
/// ```
/// In above example, `Foo` and `Bar` are both component. Foo *depends on* Bar, the dependency is
/// defined via the `#[injected]` attributes.
///
/// Note: a dependency injected by Shine has to be wrapped by `Injected` struct.
///
/// **Component with explict startup / shutdown logic**
/// ```
/// use shine::{Component, ComponentLifecycle, async_trait};
/// #[derive(Component)]
/// #[lifecycle]
/// struct Foo {
///     internal: Option<u32>
/// }
///
/// #[async_trait]
/// impl ComponentLifecycle for Foo {
///     async fn start(&mut self) {
///         self.internal = Some(42);
///     }
///     async fn stop(&mut self) {
///         // Any shutdown logic
///     }
/// }
/// ```
#[async_trait()]
pub trait Component: Send + downcast::Downcast + ComponentLifecycle {
    fn build(registry: &ComponentRepository) -> Self
        where Self: Sized;

    fn meta() -> ComponentMeta<Box<Self>>
        where Self: Sized;
}

#[doc(hidden)]
pub struct ComponentMeta<T: ?Sized> {
    pub depends_on: Vec<TypeId>,
    pub type_id: TypeId,
    pub build: Box<dyn Fn(&ComponentRepository) -> T>
}

impl<T: Component + 'static> From<ComponentMeta<Box<T>>> for ComponentMeta<Box<dyn Component>> {

    fn from(m: ComponentMeta<Box<T>>) -> Self {
        ComponentMeta {
            depends_on: m.depends_on.clone(),
            type_id: m.type_id,
            build: Box::new(move |r: &ComponentRepository| (m.build)(r))
        }
    }
}
