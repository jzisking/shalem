use bevy::prelude::Resource;
use bevy::utils::HashMap;

pub struct ItemDesc {
    pub name: String,
    pub texture: String,
    pub max_stack_size: u32,
}

#[derive(Default, Resource)]
pub struct ItemRegistry {
    items: HashMap<String, ItemDesc>,
}

impl ItemRegistry {
    pub fn register_item(&mut self, ident: &str, desc: ItemDesc) {
        self.items.insert(ident.to_owned(), desc);
    }

    pub fn item(&self, ident: &str) -> Option<&ItemDesc> {
        self.items.get(ident)
    }
}
