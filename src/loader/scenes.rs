use eyre::Result;
use molecule::prelude::{Builder, Entity};
use serde::Deserialize;
use spore_warriors_generated as generated;
use std::fs;
use std::path::PathBuf;

use crate::{convert_u16, convert_vec};

use super::types::{Context, Coordinate, GridSize, Random};

#[derive(Deserialize, Debug)]
pub struct NodeEnemy {
    pub count: u8,
    pub enemy_pool: Vec<u16>,
}

impl From<NodeEnemy> for generated::NodeInstanceUnion {
    fn from(value: NodeEnemy) -> Self {
        let NodeEnemy { count, enemy_pool } = value;
        let node = generated::NodeEnemy::new_builder()
            .count(count.into())
            .enemy_pool(convert_vec!(enemy_pool, ResourceId, ResourceIdVec))
            .build();
        generated::NodeInstanceUnion::NodeEnemy(node)
    }
}

#[derive(Deserialize, Debug)]
pub struct NodeTreasureChest {
    pub pick: u8,
    pub count: u8,
    pub item_pool: Vec<u16>,
}

impl From<NodeTreasureChest> for generated::NodeInstanceUnion {
    fn from(value: NodeTreasureChest) -> Self {
        let NodeTreasureChest {
            pick,
            count,
            item_pool,
        } = value;
        let node = generated::NodeTreasureChest::new_builder()
            .pick(pick.into())
            .count(count.into())
            .item_pool(convert_vec!(item_pool, ResourceId, ResourceIdVec))
            .build();
        generated::NodeInstanceUnion::NodeTreasureChest(node)
    }
}

#[derive(Deserialize, Debug)]
pub struct NodeItemMerchant {
    pub count: u8,
    pub item_pool: Vec<u16>,
}

impl From<NodeItemMerchant> for generated::NodeInstanceUnion {
    fn from(value: NodeItemMerchant) -> Self {
        let NodeItemMerchant { count, item_pool } = value;
        let node = generated::NodeItemMerchant::new_builder()
            .count(count.into())
            .item_pool(convert_vec!(item_pool, ResourceId, ResourceIdVec))
            .build();
        generated::NodeInstanceUnion::NodeItemMerchant(node)
    }
}

#[derive(Deserialize, Debug)]
pub struct NodeCardMerchant {
    pub count: u8,
    pub card_pool: Vec<u16>,
}

impl From<NodeCardMerchant> for generated::NodeInstanceUnion {
    fn from(value: NodeCardMerchant) -> Self {
        let NodeCardMerchant { count, card_pool } = value;
        let node = generated::NodeCardMerchant::new_builder()
            .count(count.into())
            .card_pool(convert_vec!(card_pool, ResourceId, ResourceIdVec))
            .build();
        generated::NodeInstanceUnion::NodeCardMerchant(node)
    }
}

#[derive(Deserialize, Debug)]
pub struct NodeUnknown {
    pub count: u8,
    pub system_pool: Vec<Context>,
}

impl From<NodeUnknown> for generated::NodeInstanceUnion {
    fn from(value: NodeUnknown) -> Self {
        let NodeUnknown { count, system_pool } = value;
        let node = generated::NodeUnknown::new_builder()
            .count(count.into())
            .system_pool(convert_vec!(system_pool, ContextVec))
            .build();
        generated::NodeInstanceUnion::NodeUnknown(node)
    }
}

#[derive(Deserialize, Debug)]
pub enum NodeInstance {
    #[serde(alias = "enemy")]
    Enemy(NodeEnemy),
    #[serde(alias = "treasure_chest")]
    TreasureChest(NodeTreasureChest),
    #[serde(alias = "recover_point")]
    RecoverPoint(u8),
    #[serde(alias = "item_merchant")]
    ItemMerchant(NodeItemMerchant),
    #[serde(alias = "card_merchant")]
    CardMerchant(NodeCardMerchant),
    #[serde(alias = "unknown")]
    Unknown(NodeUnknown),
    #[serde(alias = "campsite")]
    Campsite(Context),
    #[serde(alias = "barrier")]
    Barrier,
    #[serde(alias = "starting_point")]
    StartingPoint,
    #[serde(alias = "targeting_point")]
    TargetingPoint,
}

impl From<NodeInstance> for generated::NodeInstance {
    fn from(value: NodeInstance) -> Self {
        let union = match value {
            NodeInstance::Enemy(v) => v.into(),
            NodeInstance::TreasureChest(v) => v.into(),
            NodeInstance::RecoverPoint(v) => generated::NodeInstanceUnion::NodeRecoverPoint(
                generated::NodeRecoverPoint::new_builder()
                    .hp_percent(v.into())
                    .build(),
            ),
            NodeInstance::ItemMerchant(v) => v.into(),
            NodeInstance::CardMerchant(v) => v.into(),
            NodeInstance::Unknown(v) => v.into(),
            NodeInstance::Campsite(v) => generated::NodeInstanceUnion::NodeCampsite(
                generated::NodeCampsite::new_builder()
                    .card_context(v.into())
                    .build(),
            ),
            NodeInstance::Barrier => {
                generated::NodeInstanceUnion::NodeBarrier(generated::NodeBarrier::default())
            }
            NodeInstance::StartingPoint => generated::NodeInstanceUnion::NodeStartingPoint(
                generated::NodeStartingPoint::default(),
            ),
            NodeInstance::TargetingPoint => generated::NodeInstanceUnion::NodeTargetingPoint(
                generated::NodeTargetingPoint::default(),
            ),
        };
        Self::new_builder().set(union).build()
    }
}

#[derive(Deserialize, Debug)]
pub struct LevelNode {
    pub visible: bool,
    #[serde(default)]
    pub size: GridSize,
    pub instance: NodeInstance,
}

impl From<LevelNode> for generated::LevelNode {
    fn from(value: LevelNode) -> Self {
        Self::new_builder()
            .visible((value.visible as u8).into())
            .size(value.size.into())
            .node(value.instance.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct FixedLevelNode {
    pub point: Coordinate,
    pub node: LevelNode,
}

impl From<FixedLevelNode> for generated::FixedLevelNode {
    fn from(value: FixedLevelNode) -> Self {
        Self::new_builder()
            .point(value.point.into())
            .node(value.node.into())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct ScenePartition {
    pub start_point: Coordinate,
    pub end_point: Coordinate,
    pub count: Random<u8>,
    pub node_pool: Vec<LevelNode>,
}

impl From<ScenePartition> for generated::ScenePartition {
    fn from(value: ScenePartition) -> Self {
        let node_pool = value.node_pool;
        Self::new_builder()
            .start_point(value.start_point.into())
            .end_point(value.end_point.into())
            .count(value.count.into())
            .node_pool(convert_vec!(node_pool, LevelNodeVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub id: u16,
    pub width: u8,
    pub height: u8,
    pub fixed_nodes: Vec<FixedLevelNode>,
    pub partition_list: Vec<ScenePartition>,
}

impl From<Scene> for generated::MapScene {
    fn from(value: Scene) -> Self {
        let Scene {
            id,
            width,
            height,
            fixed_nodes,
            partition_list,
        } = value;
        Self::new_builder()
            .id(convert_u16!(id, ResourceId))
            .width(width.into())
            .height(height.into())
            .fixed_nodes(convert_vec!(fixed_nodes, FixedLevelNodeVec))
            .partition_list(convert_vec!(partition_list, ScenePartitionVec))
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct ScenePool {
    #[serde(alias = "scenes")]
    pub scene_pool: Vec<Scene>,
}

impl ScenePool {
    pub fn parse_from(path: PathBuf) -> Result<Self> {
        let raw_scene_pool = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw_scene_pool)?)
    }
}

impl From<ScenePool> for generated::MapSceneVec {
    fn from(value: ScenePool) -> Self {
        let pool = value.scene_pool;
        convert_vec!(pool, MapSceneVec)
    }
}

#[test]
fn test_parse_scene_pool() {
    let scene_pool = ScenePool::parse_from("./resources/scenes.json".into()).unwrap();
    println!("[RAW] scene_pool: {scene_pool:?}");
    let mol: generated::MapSceneVec = scene_pool.into();
    println!("[MOL] MapSceneVec: {mol:?}");
}
