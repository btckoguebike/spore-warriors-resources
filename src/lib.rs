use loader::{ActionPool, CardPool, EnemyPool, LootPool, ScenePool, SystemPool, WarriorPool};
use molecule::prelude::{Builder, Entity};
use spore_warriors_generated as generated;

mod loader;

pub fn parse_to_binary(
    action_pool: &str,
    card_pool: &str,
    system_pool: &str,
    enemy_pool: &str,
    loot_pool: &str,
    scene_pool: &str,
    warrior_pool: &str,
) -> eyre::Result<Vec<u8>> {
    let action_pool: ActionPool = serde_json::from_str(action_pool)?;
    let card_pool: CardPool = serde_json::from_str(card_pool)?;
    let system_pool: SystemPool = serde_json::from_str(system_pool)?;
    let enemy_pool: EnemyPool = serde_json::from_str(enemy_pool)?;
    let loot_pool: LootPool = serde_json::from_str(loot_pool)?;
    let scene_pool: ScenePool = serde_json::from_str(scene_pool)?;
    let warrior_pool: WarriorPool = serde_json::from_str(warrior_pool)?;

    let resource_pool = generated::ResourcePool::new_builder()
        .action_pool(action_pool.into())
        .card_pool(card_pool.into())
        .system_pool(system_pool.into())
        .enemy_pool(enemy_pool.into())
        .loot_pool(loot_pool.into())
        .scene_pool(scene_pool.into())
        .warrior_pool(warrior_pool.into())
        .build();

    Ok(resource_pool.as_bytes().to_vec())
}
