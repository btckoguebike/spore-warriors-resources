use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::path::PathBuf;
use std::{fmt::Debug, fs};

use crate::loader::types::Value;
use crate::{convert_opt, convert_u16, convert_vec};

#[derive(Deserialize, Debug)]
pub struct Duration {
    pub trigger: u8,
    pub count: u8,
}

impl From<Duration> for generated::Duration {
    fn from(value: Duration) -> Self {
        generated::Duration::new_builder()
            .trigger(value.trigger.into())
            .count(value.count.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct System {
    pub id: u16,
    #[serde(alias = "system")]
    pub system_id: u16,
    #[serde(default)]
    pub target_type: u8,
    pub args: Vec<Value>,
    pub duration: Option<Duration>,
}

impl From<System> for generated::System {
    fn from(value: System) -> Self {
        let id = value.id;
        let system_id = value.system_id;
        let args = generated::ValueVec::new_builder()
            .set(value.args.into_iter().map(Into::into).collect())
            .build();
        let duration = value.duration;
        generated::System::new_builder()
            .id(convert_u16!(id, ResourceId))
            .system_id(convert_u16!(system_id, SystemId))
            .target_type(value.target_type.into())
            .args(args)
            .duration(convert_opt!(duration, DurationOpt))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct SystemPool {
    #[serde(alias = "systems")]
    pub system_pool: Vec<System>,
}

impl SystemPool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_effect_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_effect_pool)?)
    }
}

impl From<SystemPool> for generated::SystemVec {
    fn from(value: SystemPool) -> Self {
        let pool = value.system_pool;
        convert_vec!(pool, SystemVec)
    }
}

#[test]
fn test_parse_system_pool() {
    let system_pool = SystemPool::parse_from("./resources/systems.json".into()).unwrap();
    println!("[RAW] system_pool: {system_pool:?}");
    let mol: generated::SystemVec = system_pool.into();
    println!("[MOL] SystemVec: {mol:?}");
}
