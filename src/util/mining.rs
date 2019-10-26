#![allow(non_snake_case)]

use core::result::Result;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Mining {}

const BASE_URL: &str = "https://rest.bitcoin.com/v2/mining/";

#[derive(Deserialize, Debug)]
pub struct MiningInfo {
    blocks: u32,
    currentblocksize: u32,
    currentblocktx: u32,
    difficulty: f64,
    blockprioritypercentage: u32,
    networkhashps: u64,
    pooledtx: u32,
    chain: String,
    warnings: String,
}

impl Mining {
    pub fn get_mining_info() -> Result<MiningInfo, Error> {
        let url: String = format!("{}getMiningInfo", BASE_URL);
        println!("{}", url);
        let s_slice: &str = &url[..];
        let info: MiningInfo = reqwest::get(s_slice)?.json()?;
        Ok(info)
    }

    pub fn get_network_hashps() -> Result<u64, Error> {
        let url: String = format!("{}getNetworkHashps", BASE_URL);
        let s_slice: &str = &url[..];
        let info: u64 = reqwest::get(s_slice)?.json()?;
        Ok(info)
    }
}
