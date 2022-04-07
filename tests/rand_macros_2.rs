extern crate rand;

use rand::Rng;
#[macro_use]
extern crate rand_derive2;

#[derive(RandGen)]
enum EnumUnit {
    X,
}

#[derive(RandGen, Debug)]
enum Enum1 {
    X(u8, f32),
}

#[derive(RandGen, Debug)]
enum Enum2 {
    X(bool),
    Y,
}

#[derive(RandGen, Debug)]
enum Enum3 {
    X { x: u8, y: isize },
    Z,
}

#[derive(RandGen, Debug)]
enum Enum4 {
    S(InnerStruct),
    B(ValType),
    X { x: u8, y: isize },
    Z,
    M{ a: InnerStruct, b: ValType },
}

#[derive(Debug, RandGen)]
struct InnerStruct {
    a: u8,
    b: i32,
    c: bool
}

#[derive(Debug, RandGen)]
pub enum ValType {
    /// The `i32` type.
    I32 = 0x7F,
    /// The `i64` type.
    I64 = 0x7E,
}

#[derive(Debug, RandGen)]
enum Enum5 {
    BrTable(Vec<u32>, u32),
}

#[test]
fn smoke() {
    let mut rng = rand::thread_rng();

    // check nothing horrible happens internally:
    for _ in 0..100 {
        let _ = rng.gen::<EnumUnit>();
        let a = rng.gen::<Enum1>();
        let a = rng.gen::<Enum2>();
        let a = rng.gen::<Enum3>();
        let a = rng.gen::<Enum4>();
        let a = rng.gen::<Enum5>();
        println!("{:?}", a);
    }
}
