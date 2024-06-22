use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct Slot {
    pub active: bool,
    pub seconds_played: u32,
    character_name: String,
    pub character_level: u32,
    pub index: usize,
}

#[wasm_bindgen]
impl Slot {
    #[wasm_bindgen(getter)]
    pub fn character_name(&self) -> String {
        self.character_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_character_name(&mut self, id: String) {
        self.character_name = id;
    }
}

#[wasm_bindgen]
pub fn get_slots(data: Vec<u8>) -> Vec<Slot> {
    elden_ring_saver::get_all_slots(&data)
        .into_iter()
        .map(|i| Slot {
            active: i.active,
            seconds_played: i.seconds_played,
            character_name: i.character_name,
            character_level: i.character_level,
            index: i.index,
        })
        .collect()
}

#[wasm_bindgen]
pub fn replace_slot(
    target: Vec<u8>,
    target_slot_index: usize,
    source: Vec<u8>,
    source_slot_index: usize,
) -> Vec<u8> {
    let target = target.clone();
    let source = source.clone();

    elden_ring_saver::replace_slot(&target, target_slot_index, &source, source_slot_index)
}
