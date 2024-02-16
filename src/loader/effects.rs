use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::path::PathBuf;
use std::{fmt::Debug, fs};

use super::types::Context;
use crate::{convert_opt, convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Effect {
    pub id: u16,
    pub trigger: Option<Context>,
    pub execution: Option<Context>,
    pub discard: Option<Context>,
}

impl From<Effect> for generated::Effect {
    fn from(value: Effect) -> Self {
        let Effect {
            id,
            trigger,
            execution,
            discard,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .trigger(convert_opt!(trigger, ContextOpt))
            .execution(convert_opt!(execution, ContextOpt))
            .discard(convert_opt!(discard, ContextOpt))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct EffectPool {
    #[serde(alias = "effects")]
    pub effect_pool: Vec<Effect>,
}

impl EffectPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_effect_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_effect_pool)?)
    }
}

impl From<EffectPool> for generated::EffectVec {
    fn from(value: EffectPool) -> Self {
        let pool = value.effect_pool;
        convert_vec!(pool, EffectVec)
    }
}

#[test]
fn test_parse_effect_pool() {
    let effect_pool = EffectPool::parse_from("./resources/effects.json".into()).unwrap();
    println!("[RAW] effect_pool: {effect_pool:?}");
    let mol: generated::EffectVec = effect_pool.into();
    println!("[MOL] EffectVec: {mol:?}");
}
