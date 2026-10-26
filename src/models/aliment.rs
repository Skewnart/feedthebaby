#[derive(Copy, Clone)]
pub struct Aliment {
    pub name: &'static str,
    pub quantity: i8,
    pub date: &'static str,
}

impl Aliment {
    pub const fn new(name: &'static str, quantity: i8, date: &'static str) -> Self {
        Self { name, quantity, date }
    }
}
