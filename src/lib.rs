use loader::{
    ActionPool, CardPool, EnemyPool, ItemPool, LootPool, ScenePool, SystemPool, WarriorPool,
};
use molecule::prelude::{Builder, Entity};
use spore_warriors_generated as generated;

mod loader;

pub fn generate_resource_binary(
    action_pool: ActionPool,
    card_pool: CardPool,
    system_pool: SystemPool,
    enemy_pool: EnemyPool,
    loot_pool: LootPool,
    scene_pool: ScenePool,
    warrior_pool: WarriorPool,
    item_pool: ItemPool,
) -> Vec<u8> {
    generated::ResourcePool::new_builder()
        .action_pool(action_pool.into())
        .card_pool(card_pool.into())
        .system_pool(system_pool.into())
        .enemy_pool(enemy_pool.into())
        .loot_pool(loot_pool.into())
        .scene_pool(scene_pool.into())
        .warrior_pool(warrior_pool.into())
        .item_pool(item_pool.into())
        .build()
        .as_bytes()
        .to_vec()
}
