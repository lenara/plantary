#![allow(unused)]

use near_sdk::{Balance};

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
pub const P_PRICES: [Balance; 7] = [
    0, // generic
    10, // oracle
    20, // portrait
    30, // money 
    0,
    0,
    0
];

// prices to harvest
pub const H_PRICES: [Balance; 7] = [
    0, // generic
    5, // oracle
    5, // portrait
    0, // money (can't harvest)
    5,
    5,
    50
];

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
    "https://arweave.net/mGhn0lNxVB6rfon61c9rRioMKL3ZsbjrVJA0qt9St4o",
    "https://arweave.net/3xMnn8J1ViLX8uHRfDxMpAZS2tSwT7VWrdjDT3fV2xQ",
    "https://arweave.net/_q0UfS76GMma9PR-XMavRI8ozipY_cmgoi6TFS_eHOg",
    "https://arweave.net/-Nxk3noWBskl8kbfxhCZFspD7v9lf79iJt1bQ2TCTzw",
    "https://arweave.net/zXIyOvf6q42eiVnixg6EK_RmFfxlZaFuaQgvs9b6Y8c",
    "https://arweave.net/u2wER7li2oXgMXfRUs22oERc-XUsn2Ph9yBsZPrvcBc",
    "https://arweave.net/eQQKfobStzP8dHIzbXYjJCMKQR1owIZ5ljjwX3xvz7I",
        ]);
        map.insert(ptypes::PORTRAIT, vec![
    "https://arweave.net/tmOUL9xwL8LQb_E5kOldLaF0mrZLg9rSMYpoTGgdkU8",
    "https://arweave.net/tvCQax-rq-oDvRdy-QnBp5orrjSP04Y-dNxXC3maTkI",
    "https://arweave.net/CJyoNeeDM_Vco0l4-7y434_pe4hBhWEE9vvh5XqMd4k",
    "https://arweave.net/hZ3etzVzsaXX6utSldyfvIvp0JUoFISuA72vJpNKa8s",
    "https://arweave.net/63vqengRMJiiBU-YmVpRH9nDclZB_f3zNVsn0wtcqg4",
    "https://arweave.net/mTFjapdnAWqWLXOqySLM0cHX2AOcYYWpoSPArBA_suk",
    "https://arweave.net/eaCk3l8Oi3MqNi7lKMRRC7gR5zRXO9JfbJc80OquHQk",
    "https://arweave.net/30kVPubXOw6vJce923j6Nv27jWl39AeS4EcpMijCmZA",
    "https://arweave.net/1xw0bRDaU-CV7hsOGnP51ZWr5_zVk21Qxu8h_jcX-tg",
    "https://arweave.net/FK4nE9euzIoEx4QOPpocSKDK0wjrPwcxX0cjxx8Km5I",
    "https://arweave.net/zIU6uG94XnwtTeEzHB3GSikBTtErmE3fgWCZV744tZE",
    "https://arweave.net/usIqVRzLyFNGENUgeV8c5-zjzEptOJZa23BkUDiU3cU",
    "https://arweave.net/EDiBwvIYUmT5cmPqbW02HOuFZnHUPoTNX_ri3N2BeTg",
    "https://arweave.net/LJkU3DnETelIpCdn6l6v-ZDUdY0LzW77G3qNPkRl7cs", // Dennis, by Ilan Katin
        ]);
        map
    };
}
