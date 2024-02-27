use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use crate::loader::types::Random;
use crate::{convert_opt, convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Package {
    pub size: u8,
    pub item_pool: Vec<u16>,
}

impl From<Package> for generated::Package {
    fn from(value: Package) -> Self {
        let Package { size, item_pool } = value;
        Self::new_builder()
            .size(size.into())
            .item_pool(convert_vec!(item_pool, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct Loot {
    pub id: u16,
    pub gold: Random<u16>,
    pub score: Random<u16>,
    pub card_pool: Package,
    pub props_pool: Option<Package>,
    pub equipment_pool: Option<Package>,
}

impl From<Loot> for generated::Loot {
    fn from(value: Loot) -> Self {
        let Loot {
            id,
            gold,
            score,
            card_pool,
            props_pool,
            equipment_pool,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .gold(gold.into())
            .score(score.into())
            .card_pool(card_pool.into())
            .props_pool(convert_opt!(props_pool, PackageOpt))
            .equipment_pool(convert_opt!(equipment_pool, PackageOpt))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct LootPool {
    #[serde(alias = "loots")]
    pub loot_pool: Vec<Loot>,
}

impl LootPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_loot_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_loot_pool)?)
    }
}

impl From<LootPool> for generated::LootVec {
    fn from(value: LootPool) -> Self {
        let pool = value.loot_pool;
        convert_vec!(pool, LootVec)
    }
}

#[test]
fn test_parse_loot_pool() {
    let loot_pool = LootPool::parse_from("./resources/loots.json".into()).unwrap();
    println!("[RAW] loot_pool: {loot_pool:?}");
    let mol: generated::LootVec = loot_pool.into();
    println!("[MOL] LootVec: {mol:?}");
}
