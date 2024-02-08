use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use crate::{convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Warrior {
    pub id: u16,
    pub special_cards: Vec<u16>,
    pub hp: u16,
    pub gold: u16,
    pub power: u8,
    pub motion: u8,
    pub view_range: u8,
    #[serde(default)]
    pub armor: u8,
    #[serde(default)]
    pub shield: u8,
    #[serde(default)]
    pub attack: u8,
    #[serde(default)]
    pub attack_weak: u8,
    #[serde(default)]
    pub defense: u8,
    #[serde(default)]
    pub defense_weak: u8,
    pub physique: u8,
    pub draw_count: u8,
    pub deck_status: Vec<u16>,
    pub package_status: Vec<u16>,
}

impl From<Warrior> for generated::Warrior {
    fn from(value: Warrior) -> Self {
        let id = value.id;
        let hp = value.hp;
        let gold = value.gold;
        let special_cards = value.special_cards;
        let deck_status = value.deck_status;
        let package_status = value.package_status;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .special_cards(convert_vec!(special_cards, ResourceId, ResourceIdVec))
            .hp(convert_u16!(hp, Number))
            .gold(convert_u16!(gold, Number))
            .power(value.power.into())
            .motion(value.motion.into())
            .view_range(value.view_range.into())
            .armor(value.armor.into())
            .shield(value.shield.into())
            .attack(value.attack.into())
            .attack_weak(value.attack_weak.into())
            .defense(value.defense.into())
            .defense_weak(value.defense_weak.into())
            .deck_status(convert_vec!(deck_status, ResourceId, ResourceIdVec))
            .package_status(convert_vec!(package_status, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct WarriorPool {
    #[serde(alias = "warriors")]
    pub warrior_pool: Vec<Warrior>,
}

impl WarriorPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_warrior_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_warrior_pool)?)
    }
}

impl From<WarriorPool> for generated::WarriorVec {
    fn from(value: WarriorPool) -> Self {
        let pool = value.warrior_pool;
        convert_vec!(pool, WarriorVec)
    }
}

#[test]
fn test_parse_warrior_pool() {
    let warrior_pool = WarriorPool::parse_from("./resources/warriors.json".into()).unwrap();
    println!("[RAW] warrior_pool: {warrior_pool:?}");
    let mol: generated::WarriorVec = warrior_pool.into();
    println!("[MOL] WarriorVec: {mol:?}");
}
