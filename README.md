# Sai

Sai is a framework for managing lifecycle and dependency of your software components.
In some languages, it was called "IoC" and "Dependency Injection".
The main usecase of this framework is on medium/large scale web services.

The Sai ecosystem consists of two major concepts: [System](struct.System.html), [Component](trait.Component.html).
A System is a runtime unit that control lifecycles of all Components.
A Component is a group of logic. A Component can depends on other Components and it can
also have its own internal state.

## FAQs

- Q: What does "Sai" mean?
- A: Nothing really. It happens to be my cat's name. I can't find a good enough name because cargo has only single namespace and many good names are reserved (yes, they are reserved rather than used).

- Q: Does this handle circular dependency?
- A: No, it does not currently.
