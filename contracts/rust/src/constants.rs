#![allow(unused)]

pub type VeggieType = i8;
pub type VeggieSubType = i8;
pub type PlantType = VeggieSubType;
pub type HarvestType = VeggieSubType;

pub mod vtypes {
    use crate::constants::VeggieType;
    pub const PLANT: VeggieType = 1;
    pub const HARVEST: VeggieType = 2;
}
// Types of plant
pub mod ptypes {
    use crate::constants::PlantType;
    pub const GENERIC: PlantType = 0;
    pub const ORACLE: PlantType = 1;
    pub const PORTRAIT: PlantType = 2;
    pub const MONEY: PlantType = 3;
}
// types of harvest
pub mod htypes {
    use crate::constants::HarvestType;
    pub const GENERIC: HarvestType= 0;
}
