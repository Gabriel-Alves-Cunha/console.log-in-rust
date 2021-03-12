macro_rules! log {
    ($($key: ident),+) => (
        $(
            println!("{:?}", $key);
        )*
    );

    ($($key: expr),+) => (
        $(
            println!("{:?}", $key);
        )*
    );
}

fn main() {
    let str = "Hello, world!";
    let num = 134567;
    log!(str);
    log!(num);
    #[derive(Debug)]
    struct Avd {
        a1: char,
        a2: i32,
        a3: f32
    }
    let hey = Avd{a1: 'x', a2: 12, a3: 32.3};
    log!(hey, num);
    log!(Avd{a1: 'x', a2: 12, a3: 32.3}, num, str);
}
