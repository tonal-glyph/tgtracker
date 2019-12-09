#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Busses {
    Master = 0,
    Bus1 = 1,
    Bus2 = 2,
    Bus3 = 3,
    Bus4 = 4,
}
impl Busses {
    pub const VARIANTS: [Busses; 5] = [
        Busses::Master,
        Busses::Bus1,
        Busses::Bus2,
        Busses::Bus3,
        Busses::Bus4,
    ];
}
impl Default for Busses {
    fn default() -> Self {
        Self::Master
    }
}
