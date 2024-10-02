//@ run-pass

enum Foo {
    Bar,
    Baz,
}

fn main() {
    'foo: match Foo::Bar {
        Foo::Bar => continue 'foo Foo::Baz,
        Foo::Baz => continue 'foo rand(),
    }
}

#[inline(never)]
fn rand() -> Foo {
    Foo::Bar // Chosen without dice roll
}
