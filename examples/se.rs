use std::fs::File;
use std::io::BufWriter;

use anyhow::Result;

use ciborium::value::Value;
use ciborium::{cbor, ser};

const FORMAT_VERSION: u32 = 0;
const WORK_NUMBER: u32 = 1;
const WORKS: u32 = 2;

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 1;

const SERIAL_KEY: u32 = 0;
const APPLE_KEY: u32 = 1;
const BANANA_KEY: u32 = 2;

const BEFORE_KEY: u32 = 0;
const AFTER_KEY: u32 = 1;

fn main() -> Result<()> {
    let work1 = to_foods(1, to_food(808.8, 800.), to_food(606.6, 600.));
    let works: Vec<Value> = (1..21)
        .map(|x| {
            to_foods(
                x as u32 + 1,
                to_food(808.8, 800.),
                to_food(606.6, 600.),
            )
        })
        .collect();

    let x = cbor!({
        FORMAT_VERSION => {
            VERSION_MAJOR => 0,
            VERSION_MINOR => 0,
        },
        WORK_NUMBER  => "22-02048",
        WORKS => [
            work1,
            works[0],
            works[1],
            works[2],
            works[3],
            works[4],
            works[5],
            works[6],
            works[7],
            works[8],
            works[9],
            works[10],
            works[11],
            works[12],
            works[13],
            works[14],
            works[15],
            works[16],
            works[17],
            works[18],
            works[18],
            works[19],
        ]
    })
    .unwrap();
    println!("{:?}", x);

    let filepath = "x.cbor";
    let writer = BufWriter::new(File::create(filepath)?);

    ser::into_writer(&x, writer)?;
    //writer.write(x)?;
    //writer.flush()?;

    Ok(())
}

fn to_foods(num: u32, apple: Value, banana: Value) -> Value {
    cbor!({
        SERIAL_KEY => num,
        APPLE_KEY => apple,
        BANANA_KEY => banana,
    })
    .unwrap()
}

fn to_food(before: f32, after: f32) -> Value {
    cbor!({
        BEFORE_KEY => before,
        AFTER_KEY => after,
    })
    .unwrap()
}
