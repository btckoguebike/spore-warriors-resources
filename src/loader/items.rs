use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use super::types::Random;
use crate::{convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u16,
    pub class: u8,
    pub quality: u8,
    pub random_weight: Random<u8>,
    pub price: Random<u16>,
    pub system_pool: Vec<u16>,
}

impl From<Item> for generated::Item {
    fn from(value: Item) -> Self {
        let Item {
            id,
            class,
            quality,
            random_weight,
            price,
            system_pool,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .class(class.into())
            .quality(quality.into())
            .random_weight(random_weight.into())
            .price(price.into())
            .system_pool(convert_vec!(system_pool, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct ItemPool {
    #[serde(alias = "items")]
    pub item_pool: Vec<Item>,
}

impl ItemPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_item_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_item_pool)?)
    }
}

impl From<ItemPool> for generated::ItemVec {
    fn from(value: ItemPool) -> Self {
        let pool = value.item_pool;
        convert_vec!(pool, ItemVec)
    }
}

#[test]
fn test_parse_item_pool() {
    let item_pool = ItemPool::parse_from("./resources/items.json".into()).unwrap();
    println!("[RAW] item_pool: {item_pool:?}");
    let mol: generated::ItemVec = item_pool.into();
    println!("[MOL] ItemVec: {mol:?}");
}
