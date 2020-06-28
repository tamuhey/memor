# memor: Simple memoization macro for rust

## Usage

Just add `#[memo]` to your function.

```rust
use memor::memo;
#[memo]
fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

assert_eq!(12586269025, fib(50));
```

Various functions can be memoized.
More Precisely, this macro can be applied to functions all of whose arguments implement `Eq + Hash`
because the arguments are saved as the keys of `std::collections::HashMap` internally,


```rust
use memor::memo;
#[derive(Hash, Eq, PartialEq)]
struct Foo {
    a: usize,
    b: usize,
}

#[memo]
fn foo(Foo { a, b }: Foo, c: usize) -> usize {
    if a == 0 || b == 0 || c == 0 {
        1
    } else {
        foo(Foo { a, b: b - 1 }, c)
            .wrapping_add(foo(Foo { a: a - 1, b }, c))
            .wrapping_add(foo(Foo { a, b }, c - 1))
    }
}

assert_eq!(foo(Foo { a: 50, b: 50 }, 50), 6753084261833197057);
```
