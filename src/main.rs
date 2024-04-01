use molecule::prelude::{Builder, Entity};
use spore_warriors_generated as generated;

mod loader;
use loader::*;

macro_rules! json_parse {
    ($pool:ty, $name:expr) => {
        <$pool>::parse_from(format!("./resources/{}.json", $name).into())
            .expect(format!("load {}.json", $name).as_str())
    };
}

fn main() {
    let action_pool = json_parse!(ActionPool, "actions");
    let card_pool = json_parse!(CardPool, "cards");
    let system_pool = json_parse!(SystemPool, "systems");
    let enemy_pool = json_parse!(EnemyPool, "enemies");
    let loot_pool = json_parse!(LootPool, "loots");
    let scene_pool = json_parse!(ScenePool, "scenes");
    let warrior_pool = json_parse!(WarriorPool, "warriors");
    let item_pool = json_parse!(ItemPool, "items");

    let resource_pool = generated::ResourcePool::new_builder()
        .action_pool(action_pool.into())
        .card_pool(card_pool.into())
        .system_pool(system_pool.into())
        .enemy_pool(enemy_pool.into())
        .loot_pool(loot_pool.into())
        .scene_pool(scene_pool.into())
        .warrior_pool(warrior_pool.into())
        .item_pool(item_pool.into())
        .build();

    std::fs::write("./resources.bin", resource_pool.as_bytes()).expect("persistence");
}
