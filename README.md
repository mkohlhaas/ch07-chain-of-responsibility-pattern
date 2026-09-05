### The Chain of Responsibility Pattern

The Chain of Responsibility pattern allows us to process requests through a series of handlers, with
each handler deciding either to process the request or to pass it to the next handler in the chain.
This pattern decouples senders from receivers, giving multiple objects the opportunity to handle a
request without the sender knowing which object will ultimately process it.

In Rust, the Chain of Responsibility pattern is a behavioral design pattern
implemented using traits for interfaces, Option<Box<dyn Handler>> pointers for
recursive runtime linking, and enums for flexible requests:

1. Request (enum) - what has to be handled
2. Handler (trait) - `handle(…)` which matches on Request enums and `set_next_handler(…)` methods
3. Concrete Handlers (struct - implementing Handlers with a next_handler field)

### When to use the Chain of Responsibility pattern

The Chain of Responsibility pattern is particularly useful when multiple objects can handle a
request, but the handler isn't known in advance; when you want to issue a request to one of several
objects without specifying the receiver explicitly; or when the set of handlers can be configured
dynamically.
