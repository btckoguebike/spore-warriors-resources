use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use crate::{convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct ActionContext {
    pub random: bool,
    pub action_pool: Vec<u16>,
}

impl From<ActionContext> for generated::ActionContext {
    fn from(value: ActionContext) -> Self {
        let ActionContext {
            random,
            action_pool,
        } = value;
        Self::new_builder()
            .random((random as u8).into())
            .action_pool(convert_vec!(action_pool, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct Enemy {
    pub id: u16,
    pub rank: u8,
    pub hp: u16,
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
    pub loot_pool: Vec<u16>,
    pub action_strategy: ActionContext,
}

impl From<Enemy> for generated::Enemy {
    fn from(value: Enemy) -> Self {
        let id = value.id;
        let hp = value.hp;
        let loot_pool = value.loot_pool;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .rank(value.rank.into())
            .hp(convert_u16!(hp, Number))
            .armor(value.armor.into())
            .shield(value.shield.into())
            .attack(value.attack.into())
            .attack_weak(value.attack_weak.into())
            .defense(value.defense.into())
            .defense_weak(value.defense_weak.into())
            .loot_pool(convert_vec!(loot_pool, ResourceId, ResourceIdVec))
            .action_strategy(value.action_strategy.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct EnemyPool {
    #[serde(alias = "enemies")]
    pub enemy_pool: Vec<Enemy>,
}

impl From<EnemyPool> for generated::EnemyVec {
    fn from(value: EnemyPool) -> Self {
        let pool = value.enemy_pool;
        convert_vec!(pool, EnemyVec)
    }
}

impl EnemyPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_enemy_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_enemy_pool)?)
    }
}

#[test]
fn test_parse_enemy_pool() {
    let enemy_pool = EnemyPool::parse_from("./resources/enemies.json".into()).unwrap();
    println!("[RAW] enemy_pool: {enemy_pool:?}");
    let mol: generated::EnemyVec = enemy_pool.into();
    println!("[MOL] EnemyVec: {mol:?}");
}
