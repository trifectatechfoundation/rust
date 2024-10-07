//@ run-pass
//@ pretty-expanded FIXME #23616

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
    std::process::exit(0)//Foo::Bar // Chosen without dice roll
}
