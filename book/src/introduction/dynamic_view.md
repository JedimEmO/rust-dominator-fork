# The basic of FRP with futures-signals

Now that we have made a simple static html node, we'll soon want to make it a bit more dynamic.
But before we dive into the code, we'll very briefly go over the fundamental principle of functional reactive programming (FRP).

The most important principle to understand is that in FRP, we consider the view to be a functional mapping of the state.
Secondly, we consider the state to be a stream of values, not just a single value held in memory.

What does this mean?

Imagine that you have a variable `x` that holds the value `5`, and we want to turn it into the text `"5"`.

One way of doing this, of course, is to simply call `x.to_string()`.
This gives us the string representation of `x` at the time of the call.
This, however, is not very useful if we want to keep the text up to date with the value of `x`.
If we reassign a new value to x, the string representation will remain the same old `"5"` as it was before.

Imagine now that instead of `x` holding the single value `5`, it is a stream of i32 values.
We can then map this stream to a stream of strings by calling `x.map(|x| x.to_string())`.
This gives us a new stream, which will yield the string representation of the latest value of `x` whenever `x` yields a new value.

This is the basic idea of FRP.
We can combine these functional mappings over streams of state to create a dynamic view, that will always represent the latest state!

This is where `futures_signals` comes in.
It provides us with the tools needed to represent both the state and the sequence of changes to it in the form of signals!

For our first simple examples, we will be using the `Mutable<T>` type from `futures_signals`.
You can think of this as a storage box that holds your value of type T.
But additionally, you can create a signal from it that will yield the latest value inside the box whenever it is changed.

To recreate our `x` example from before, we can do the following:

```rust
let x = Mutable::new(5);
let x_string = x.signal().map(|x| x.to_string());
x.set(6);
```

A very important thing to be aware of is that the signal will only yield the latest value when it is polled.
Signals work like futures, and if you do not poll them, they do absolutely nothing!

Luckily for us, in the majority of cases, DOMINATOR will take care of this for us.
It typically has a `_signal` or `_signal_vec` (more about vecs and maps later) alternatives to its normal methods, to which you can pass the corresponding signal.

For instance, if we wanted to create a span that always represents the latest value of `x`, we could do the following:

```rust
fn span_with_x(x: Mutable<i32>) -> Dom {
    let x_string_signal = x.signal().map(|x| x.to_string());
    html!("span", {
        .text_signal(x_string_signal)
    })
}
```

This span will now always display the latest value seen from the `x_string_signal` without us having to do anything else!

### Manually polling

Sometimes, however, we may want to perform side effects based on signal changes.
There are several ways to do this, but the easiest and most straight forward way is to use `for_each()` inside a future:

```rust
let x = Mutable::new(0);
let x_sig = x.signal();

async move {
    x_sig.for_each(|v| debug!("x is: {}", x)).await;
};
```

This will poll the x signal and log new values whenever the signal is emitted.

### More in depth on signals

We barely scratched the introductory surface to signals and FRP in this section.
In later chapters, we will dive deeper into the

## Dynamic view with futures signals FRP

We can put this knowledge to use to create a dynamic view of our hello world example.
Let's make a button that, when clicked, changes the text displayed in the header:

```rust
use dominator::{append_dom, body, clone, events, html};
use futures_signals::signal::Mutable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    let our_text_string = Mutable::new("Hello, world!".to_string());

    append_dom(&body(), html!("div", {
        .child(html!("h1", {
            .text_signal(our_text_string.signal_cloned())
        }))
        .child(html!("button", {
            .text("Click me!")
            .event(clone!(our_text_string => move |_: events::Click| {
                our_text_string.set("You clicked!".to_string());
            }))
        }))
    }));
}
```

There are some new concepts here.
First, let's look at the `clone!` macro.

This is a quality of life macro provided by dominator, which lets us clone and capture a set of variables into a closure.
Without it, we would have to write something like this:

```rust
let our_text_string = Mutable::new("Hello, world!".to_string());
let our_text_string_cloned = our_text_string.clone();

html!("button", {
    .text("Click me!")
    .event(move |_: events::Click| {
        our_text_string_cloned.set("You clicked!".to_string());
    })
});
```

We also now see that the `.event()` method allows us to register an event handler on our dom nodes.
It's important to be aware that the callback we give to the event handler must be `'static`, meaning it can only capture our state by value, or by a leaked static reference to it.
References to any other lifetime is not allowed, as we cannot guarantee that the reference will be valid when the event is fired.

In this case, we are registering a handler for the `Click` event, which will set the text to a new value.

Also notice that there is no explicit updating of the dom happening in our code here.
The only thing we do is mutate the base state, and the `text_signal` will take care of updating the dom for us.
