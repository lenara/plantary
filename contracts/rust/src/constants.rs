#![allow(unused)]

pub type VeggieType = u8;
pub type VeggieSubType = u8;
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
    //pub const GENERIC: PlantType = 0;
    pub const ORACLE: PlantType = 1;
    pub const PORTRAIT: PlantType = 2;
    pub const MONEY: PlantType = 3;
    pub const COMPLIMENT: PlantType = 4;
    pub const INSULT: PlantType = 5;
    pub const SEED: PlantType = 6;
}
// types of harvest
pub mod htypes {
    use crate::constants::HarvestType;
    pub const GENERIC: HarvestType= 0;
}
// prices to harvest
pub mod hprices [
    =1, // generic
    5, // oracle
    5, // portrait
    -1, // money (can't harvest)
    5,
    5,
    50
]

// nested array of meta_urls for possible plants!
// array index == PlantType (an int)
// (for demo only ... this should be a web data struct someplace ...)

// lazy global statics, from https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // pool of possible plants, sorted by plant type
    pub static ref P_POOL: HashMap<u8, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(ptypes::ORACLE, vec![
    "https://3bvdryfdm3sswevmvr3poka2ucda5dfqag3bz4td72affctbmaea.arweave.net/2Go44KNm5SsSrKx29ygaoIYOjLABthzyY_6AUophYAg", 
    "https://vwanp7rn32rioq6ofcvglo52sgdrctcfkc4v7uiy7bbimtzijz3q.arweave.net/rYDX_i3eoodDziiqZbu6kYcRTEVQuV_RGPhChk8oTnc",
        ]);
        map.insert(ptypes::PORTRAIT, vec![
    "https://rsigfpny3j3uwohxfeo7tdkdvw6yhaefxt6d3uq7kajtpaqtdfwq.arweave.net/jJBivbjad0s49ykd-Y1Drb2DgIW8_D3SH1ATN4ITGW0",
    "https://arweave.net/fo--Wlh83Ka83zVQqliiwFq_4zbc1H7vrZNlvA_Gkek",
        ]);
        map.insert(ptypes::MONEY, vec![
    "https://rj32ukhcq4hdq7nux3rntp5ffdk3ff2kzjcalpy3mc7batjytoza.arweave.net/ineqKOKHDjh9tL7i2b-lKNWyl0rKRAW_G2C-EE04m7I",
    "https://b2zjlf2zplj5we2bdar6p6smu3o6fdu7o7ed23takt63lck6peoq.arweave.net/DrKVl1l609sTQRgj5_pMpt3ijp93yD1uYFT9tYleeR0",
        ]);
        map
    };
    // pool of possible harvests, sorted by plant type
    pub static ref H_POOL: HashMap<u8, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(ptypes::ORACLE, vec![
    "https://arweave.net/v63RbTVHhGKr7UNMmwMjBtKepk1I26UB4yxPhJVSkcg",
    "https://arweave.net/hvOKZAw3miEA8BE4VewzH9io4fNsSWyZpGZaSmhr-l8",
    "https://arweave.net/B_c8uZaUFIA8hjLDVr3v4IR6aRT-zzvCaE0cqWgVURc",
        ]);
        map.insert(ptypes::PORTRAIT, vec![
    "https://arweave.net/tmOUL9xwL8LQb_E5kOldLaF0mrZLg9rSMYpoTGgdkU8",
    "https://arweave.net/tvCQax-rq-oDvRdy-QnBp5orrjSP04Y-dNxXC3maTkI",
    "https://arweave.net/CJyoNeeDM_Vco0l4-7y434_pe4hBhWEE9vvh5XqMd4k",
        ]);
        map
    };
}
