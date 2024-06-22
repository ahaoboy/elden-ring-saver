pub const SLOT_START_INDEX: usize = 0x310;
pub const SLOT_LENGTH: usize = 0x280000;
pub const SAVE_HEAD_S_SECTION_START_INDEX: usize = 0x19003B0;
pub const SAVE_HEAD_S_SECTION_LENGTH: usize = 0x60000;
pub const SAVE_HEAD_START_INDEX: usize = 0x1901D0E;
pub const SAVE_HEAD_LENGTH: usize = 0x24C;
pub const CHAR_ACTIVE_STATUS_START_INDEX: usize = 0x1901D04;
pub const CHAR_NAME_LENGTH: usize = 0x22;
pub const CHAR_LEVEL_LOCATION: usize = 0x22;
pub const CHAR_PLAYED_START_INDEX: usize = 0x26;
pub const ID_LOCATION: usize = 0x19003B4;
pub const MAX_SLOT_SIZE: usize = 10;

pub fn get_slot_range(slot_index: usize) -> std::ops::Range<usize> {
    let st = slot_start_index(slot_index);
    st..st + SAVE_HEAD_LENGTH
}

pub fn get_active(data: &[u8], slot_index: usize) -> u8 {
    data[CHAR_ACTIVE_STATUS_START_INDEX + slot_index]
}

pub fn get_mut_active(data: &mut [u8], slot_index: usize) -> &mut u8 {
    &mut data[CHAR_ACTIVE_STATUS_START_INDEX + slot_index]
}
pub fn get_character_name(data: &[u8], slot_index: usize) -> &[u8] {
    let start = get_slot_start(slot_index);
    let end = start + CHAR_NAME_LENGTH;
    &data[start..end]
}

pub fn get_mut_character_name(data: &mut [u8], slot_index: usize) -> &mut [u8] {
    let start = get_slot_start(slot_index);
    let end = start + CHAR_NAME_LENGTH;
    &mut data[start..end]
}

pub fn get_slot_start(slot_index: usize) -> usize {
    SAVE_HEAD_START_INDEX + slot_index * SAVE_HEAD_LENGTH
}
pub fn get_character_level(data: &[u8], slot_index: usize) -> &u8 {
    let start = get_slot_start(slot_index);
    &data[start + CHAR_LEVEL_LOCATION]
}
pub fn get_mut_character_level(data: &mut [u8], slot_index: usize) -> &mut u8 {
    let start = get_slot_start(slot_index);
    &mut data[start + CHAR_LEVEL_LOCATION]
}
pub fn get_seconds_played(data: &[u8], slot_index: usize) -> &[u8] {
    let start = get_slot_start(slot_index);
    &data[start + CHAR_PLAYED_START_INDEX..start + CHAR_PLAYED_START_INDEX + 4]
}
pub fn get_mut_seconds_played(data: &mut [u8], slot_index: usize) -> &mut [u8] {
    let start = get_slot_start(slot_index);
    &mut data[start + CHAR_PLAYED_START_INDEX..start + CHAR_PLAYED_START_INDEX + 4]
}

#[derive(Debug, Clone)]
pub struct Slot {
    pub active: bool,
    pub seconds_played: u32,
    pub character_name: String,
    pub character_level: u32,
    pub index: usize,
}
fn slot_start_index(index: usize) -> usize {
    SLOT_START_INDEX + (index * 0x10) + (index * SLOT_LENGTH)
}
fn head_start_index(index: usize) -> usize {
    SAVE_HEAD_START_INDEX + (index * SAVE_HEAD_LENGTH)
}

pub fn get_save_data_range(slot_index: usize) -> std::ops::Range<usize> {
    SLOT_START_INDEX + slot_index * 0x10 + slot_index * SLOT_LENGTH
        ..SLOT_START_INDEX + slot_index * 0x10 + (slot_index + 1) * SLOT_LENGTH
}

pub fn get_head_data_range(slot_index: usize) -> std::ops::Range<usize> {
    let st = head_start_index(slot_index);
    st..st + SAVE_HEAD_LENGTH
}
pub fn get_save_data(data: &[u8], slot_index: usize) -> &[u8] {
    &data[get_save_data_range(slot_index)]
}
pub fn get_mut_save_data(data: &mut [u8], slot_index: usize) -> &mut [u8] {
    &mut data[get_save_data_range(slot_index)]
}
pub fn get_head_data(data: &[u8], slot_index: usize) -> &[u8] {
    &data[get_head_data_range(slot_index)]
}
pub fn get_mut_head_data(data: &mut [u8], slot_index: usize) -> &mut [u8] {
    &mut data[get_head_data_range(slot_index)]
}

fn find_all_slice(data: &[u8], source_id: &[u8]) -> Vec<usize> {
    let mut indices = Vec::new();
    let source_id_len = source_id.len();

    if source_id_len == 0 {
        return indices;
    }

    for i in 0..=(data.len() - source_id_len) {
        if data[i..i + source_id_len] == *source_id {
            indices.push(i);
        }
    }

    indices
}

pub fn get_steam_id_range() -> std::ops::Range<usize> {
    ID_LOCATION..ID_LOCATION + 8
}

pub fn set_steam_id(source: &mut [u8], slot_index: usize, source_id: &[u8], target_id: &[u8]) {
    let save_data = get_mut_save_data(source, slot_index);
    for i in find_all_slice(save_data, source_id) {
        save_data[i..i + 8].copy_from_slice(target_id);
    }
}

pub fn get_slot_hash_range(slot_index: usize) -> std::ops::Range<usize> {
    slot_start_index(slot_index) - 0x10..slot_start_index(slot_index)
}

pub fn get_head_hash_range() -> std::ops::Range<usize> {
    SAVE_HEAD_S_SECTION_START_INDEX - 0x10..SAVE_HEAD_S_SECTION_START_INDEX
}

pub fn get_slot(data: &[u8], slot_index: usize) -> Option<Slot> {
    if data.len() < SAVE_HEAD_START_INDEX + slot_index * SAVE_HEAD_LENGTH {
        return None;
    }

    let active = get_active(data, slot_index) == 1;

    let character_name = String::from_utf16_lossy(
        &get_character_name(data, slot_index)
            .chunks(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect::<Vec<u16>>(),
    )
    .trim_end_matches('\0')
    .to_owned();

    let character_level = *get_character_level(data, slot_index) as u32;

    let seconds_played = u32::from_le_bytes(
        get_seconds_played(data, slot_index)
            .try_into()
            .unwrap_or([0; 4]),
    );

    Some(Slot {
        active,
        character_name,
        index: slot_index,
        character_level,
        seconds_played,
    })
}

pub fn get_all_slots(data: &[u8]) -> Vec<Slot> {
    (0..MAX_SLOT_SIZE)
        .filter_map(|i| get_slot(data, i))
        .collect()
}

pub fn replace_slot(
    target: &[u8],
    target_slot_index: usize,
    source: &[u8],
    source_slot_index: usize,
) -> Vec<u8> {
    let mut new_data = target.to_vec().clone();
    let new_bytes = new_data.as_mut_slice();

    let target_id = &target[ID_LOCATION..ID_LOCATION + 8];
    let source_id = &source[ID_LOCATION..ID_LOCATION + 8];
    let target_name = get_character_name(target, target_slot_index);
    let source_name = get_character_name(source, source_slot_index);

    // save data
    let source_save_data = get_save_data(source, source_slot_index);
    new_bytes[get_save_data_range(target_slot_index)].copy_from_slice(source_save_data);

    // steam id
    set_steam_id(new_bytes, target_slot_index, source_id, target_id);

    // head data
    let head_data = &source[get_head_data_range(source_slot_index)];
    new_bytes[get_head_data_range(target_slot_index)].copy_from_slice(head_data);

    // set active
    *get_mut_active(new_bytes, target_slot_index) = 0x01;

    // name
    for i in find_all_slice(new_bytes, source_name) {
        new_bytes[i..i + source_name.len()].copy_from_slice(target_name);
    }

    // slot hash
    let slot_hash = md5::compute(get_save_data(new_bytes, target_slot_index)).to_vec();
    new_bytes[get_slot_hash_range(target_slot_index)].copy_from_slice(&slot_hash);

    // head hash
    let head_hash = md5::compute(get_head_data(new_bytes, target_slot_index)).to_vec();
    new_bytes[get_head_hash_range()].copy_from_slice(&head_hash);

    new_data
}

#[cfg(test)]
mod test {
    use crate::{get_all_slots, replace_slot};

    #[test]
    fn get_name() {
        let source = std::fs::read("../assets/source.sl2").unwrap();
        let slot = get_all_slots(&source);

        for i in slot {
            println!("{:?}", i)
        }
        let target = std::fs::read("../assets/empty.sl2").unwrap();

        let slot = get_all_slots(&target);

        for i in slot {
            println!("{:?}", i)
        }

        let target_slot_index = 0;
        let source_slot_index = 5;
        replace_slot(&target, target_slot_index, &source, source_slot_index);
    }
}
