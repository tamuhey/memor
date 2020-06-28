use memor::memo;

#[memo]
fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[test]
fn test_fib() {
    assert_eq!(12586269025, fib(50));
}

#[memo]
fn comb(n: i64, m: i64) -> i64 {
    if m == 0 {
        1
    } else if n < m {
        0
    } else {
        comb(n - 1, m - 1) + comb(n - 1, m)
    }
}

#[test]
fn test_comb() {
    assert_eq!(126410606437752, comb(50, 25));
}

#[memo]
fn ftup((a, b): (usize, usize), c: usize) -> usize {
    if a * b * c == 0 {
        1
    } else {
        ftup((a - 1, b), c)
            .wrapping_add(ftup((a, b - 1), c))
            .wrapping_add(ftup((a, b), c - 1))
    }
}

#[test]
fn test_ftup() {
    assert_eq!(ftup((30, 30), 30), 16767162301104664577);
}

#[derive(Hash, Eq, PartialEq)]
struct Foo {
    a: usize,
    b: usize,
}

#[memo]
fn foo(Foo { a, b }: Foo) -> usize {
    if a * b == 0 {
        1
    } else {
        foo(Foo { a, b: b - 1 }).wrapping_add(foo(Foo { a: a - 1, b }))
    }
}

#[test]
fn test_foo() {
    assert_eq!(foo(Foo { a: 50, b: 50 }), 1184508333840160104);
}
