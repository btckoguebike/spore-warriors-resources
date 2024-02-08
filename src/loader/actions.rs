use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use crate::{convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Action {
    pub id: u16,
    pub random: bool,
    pub effect_pool: Vec<u16>,
}

impl From<Action> for generated::Action {
    fn from(value: Action) -> Self {
        let Action {
            id,
            random,
            effect_pool,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .random((random as u8).into())
            .effect_pool(convert_vec!(effect_pool, ResourceId, ResourceIdVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct ActionPool {
    #[serde(alias = "actions")]
    pub action_pool: Vec<Action>,
}

impl ActionPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_action_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_action_pool)?)
    }
}

impl From<ActionPool> for generated::ActionVec {
    fn from(value: ActionPool) -> Self {
        let pool = value.action_pool;
        convert_vec!(pool, ActionVec)
    }
}

#[test]
fn test_parse_action_pool() {
    let action_pool = ActionPool::parse_from("./resources/actions.json".into()).unwrap();
    println!("[RAW] action_pool: {action_pool:?}");
    let mol: generated::ActionVec = action_pool.into();
    println!("[MOL] ActionVec: {mol:?}");
}
