use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;

#[macro_export]
macro_rules! convert_u16 {
    ($val:ident, $gen:ident) => {
        generated::$gen::new_builder()
            .set($val.to_le_bytes().map(Into::into))
            .build()
    };
}

#[macro_export]
macro_rules! convert_opt {
    ($val:ident, $gen:ident) => {
        generated::$gen::new_builder()
            .set($val.map(Into::into))
            .build()
    };
}

#[macro_export]
macro_rules! convert_vec {
    ($val:ident, $gen:ident) => {
        generated::$gen::new_builder()
            .set($val.into_iter().map(Into::into).collect())
            .build()
    };
    ($val:ident, $gen:ident, $vgen:ident) => {
        generated::$vgen::new_builder()
            .set(
                $val.into_iter()
                    .map(|v| crate::convert_u16!(v, $gen))
                    .collect(),
            )
            .build()
    };
}

#[derive(Deserialize, Debug)]
pub struct Random<T: Sized> {
    pub min: T,
    pub max: T,
}

impl From<Random<u16>> for generated::RandomNumber {
    fn from(value: Random<u16>) -> Self {
        let min = value.min;
        let max = value.max;
        Self::new_builder()
            .lower_bound(convert_u16!(min, Number))
            .upper_bound(convert_u16!(max, Number))
            .build()
    }
}

impl From<Random<u8>> for generated::RandomByte {
    fn from(value: Random<u8>) -> Self {
        Self::new_builder()
            .lower_bound(value.min.into())
            .upper_bound(value.max.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct GridSize {
    pub x: u8,
    pub y: u8,
}

impl From<GridSize> for generated::Size {
    fn from(value: GridSize) -> Self {
        Self::new_builder()
            .x(value.x.into())
            .y(value.y.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl From<Coordinate> for generated::Coordinate {
    fn from(value: Coordinate) -> Self {
        Self::new_builder()
            .x(value.x.into())
            .y(value.y.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub enum Value {
    #[serde(alias = "resource")]
    Resource(u16),
    #[serde(alias = "system")]
    System(u16),
    #[serde(alias = "number")]
    Number(u16),
    #[serde(alias = "random")]
    Random(Random<u16>),
}

impl From<Value> for generated::Value {
    fn from(value: Value) -> Self {
        let union = match value {
            Value::System(v) => generated::ValueUnion::SystemId(convert_u16!(v, SystemId)),
            Value::Number(v) => generated::ValueUnion::Number(convert_u16!(v, Number)),
            Value::Resource(v) => generated::ValueUnion::ResourceId(convert_u16!(v, ResourceId)),
            Value::Random(v) => generated::ValueUnion::RandomNumber(v.into()),
        };
        Self::new_builder().set(union).build()
    }
}

#[derive(Deserialize, Debug)]
pub struct Context {
    pub system: u16,
    pub args: Vec<Value>,
}

impl From<Context> for generated::Context {
    fn from(value: Context) -> Self {
        let system_id = value.system;
        let args = generated::ValueVec::new_builder()
            .set(value.args.into_iter().map(Into::into).collect())
            .build();
        generated::Context::new_builder()
            .system_id(convert_u16!(system_id, SystemId))
            .args(args)
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct LifePoint {
    pub system: u16,
    pub point: u8,
    pub round_recover: bool,
}

impl From<LifePoint> for generated::LifePoint {
    fn from(value: LifePoint) -> Self {
        let system_id = value.system;
        Self::new_builder()
            .listen_system_id(convert_u16!(system_id, SystemId))
            .point(value.point.into())
            .round_recover((value.round_recover as u8).into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub enum Duration {
    #[serde(alias = "round")]
    Round(u16),
    #[serde(alias = "life_point")]
    LifePoint(LifePoint),
}

impl From<Duration> for generated::Duration {
    fn from(value: Duration) -> Self {
        let union = match value {
            Duration::LifePoint(v) => generated::DurationUnion::LifePoint(v.into()),
            Duration::Round(v) => generated::DurationUnion::Number(convert_u16!(v, Number)),
        };
        Self::new_builder().set(union).build()
    }
}
