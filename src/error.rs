#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{} value is out of range (min: {} - max: {})", _0, _1, _2)]
    OutOfRange(u32, u32, u32),
}
