#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Duration(u32),
    Unit(TimeUnit),
    Unknown(String),
}

#[derive(Debug)]
pub enum TimeUnit {
    Day,
    Week,
    Month,
    Year,
}
