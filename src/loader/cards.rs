use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use super::types::Random;
use crate::{convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Card {
    pub id: u16,
    pub class: u8,
    pub power_cost: u8,
    pub price: Random<u16>,
    pub system_pool: Vec<u16>,
}

impl From<Card> for generated::Card {
    fn from(value: Card) -> Self {
        let Card {
            id,
            class,
            power_cost,
            price,
            system_pool,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .class(class.into())
            .cost(power_cost.into())
            .price(price.into())
            .system_pool(convert_vec!(system_pool, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct CardPool {
    #[serde(alias = "cards")]
    pub card_pool: Vec<Card>,
}

impl CardPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_card_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_card_pool)?)
    }
}

impl From<CardPool> for generated::CardVec {
    fn from(value: CardPool) -> Self {
        let pool = value.card_pool;
        convert_vec!(pool, CardVec)
    }
}

#[test]
fn test_parse_card_pool() {
    let card_pool = CardPool::parse_from("./resources/cards.json".into()).unwrap();
    println!("[RAW] card_pool: {card_pool:?}");
    let mol: generated::CardVec = card_pool.into();
    println!("[MOL] CardVec: {mol:?}");
}
