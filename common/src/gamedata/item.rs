
use array2d::Vec2d;
use objholder::ItemIdx;
use std::cmp::{PartialOrd, Ord, Ordering};
use super::defs::ElementArray;

/// Game item
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Item {
    pub idx: ItemIdx,
    pub kind: ItemKind,
    pub flags: ItemFlags,
    pub rank: ItemRank,
    pub attributes: Vec<ItemAttribute>,
}

/// ItemObject has detail data for one item
#[derive(Serialize, Deserialize)]
pub struct ItemObject {
    pub id: String,
    pub img: ::obj::Img,
    pub kind: ItemKind,
    /// Defalut item flags
    /// They are set at making object based on object setting files
    pub default_flags: ItemFlags,
    pub basic_price: u32,
    /// Item weight (gram)
    pub w: u32,
    /// The frequency of item generation in random map
    pub gen_weight: f32,
    /// The frequency of item generation in shops
    pub store_weight: f32,
    /// Generation level
    /// If it is higher, and the item will be generated on deeper floors.
    /// This parameter will be used for shops also.
    pub gen_level: u32,
    pub dice_n: u16,
    pub dice_x: u16,
    /// Defence
    pub def: ElementArray<u16>,
    /// Effectiveness of this item
    pub eff: u16,
    pub medical_effect: MedicalEffect,
    /// Character's nutrition will be increased by this value after eating this item
    pub nutrition: u16,
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        let order = self.kind.cmp(&other.kind);
        if order != Ordering::Equal { return order; }
        let order = self.idx.cmp(&other.idx);
        if order != Ordering::Equal { return order; }
        let order = self.rank.cmp(&other.rank);
        if order != Ordering::Equal { return order; }
        self.attributes.cmp(&other.attributes)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This is mainly used for item list sorting
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Object, Potion, Food, Weapon(WeaponKind), Armor(ArmorKind), Material, Special
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum ItemKindRough {
    Object, Potion, Food, Weapon, Armor, Material, Special
}

bitflags! {
    pub struct ItemFlags: u64 {
        const EATABLE   = 1 << 0;
        const DRINKABLE = 1 << 1;
    }
}

/// Item rank is used to calculate the effects.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ItemRank {
    /// Base rank of the item
    pub base: i8,
    /// This rank will be changed by some magical effects
    pub enchant: i8,
    /// If the item is damaged, this value will decrease
    pub damage: i8,
}

impl Default for ItemRank {
    fn default() -> ItemRank {
        ItemRank {
            base: 0,
            enchant: 0,
            damage: 0,
        }
    }
}

impl ItemRank {
    /// Return the summation of rank values
    pub fn as_int(&self) -> i32 {
        self.base as i32 + self.enchant as i32 + self.damage as i32
    }
}

/// Items can have zero or more attributes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemAttribute {
    /// Data to generate the contents.
    /// Used to fix generated contents when this item is opened.
    ContentGen { level: u32, seed: u32 },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum MedicalEffect {
    None, Heal, Sleep, Poison,
}

impl Default for MedicalEffect {
    fn default() -> MedicalEffect {
        MedicalEffect::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum WeaponKind {
    Sword, Spear, Axe, Whip,
    Bow, Crossbow, Gun,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum ArmorKind {
    Body, Shield,
}

/// Data to generate an item.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ItemGen {
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemListLocation {
    OnMap { mid: super::map::MapId, pos: Vec2d },
    Chara { cid: super::chara::CharaId },
    Equip { cid: super::chara::CharaId },
    Shop { cid: super::CharaId },
}

pub type ItemLocation = (ItemListLocation, u32);

/// Item list that records all items owned by one character or one tile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemList {
    pub items: Vec<(Item, u32)>,
}

impl ItemList {
    pub fn new() -> ItemList {
        ItemList {
            items: Vec::new(),
        }
    }

    /// Get the number of item
    pub fn get_number(&self, i: u32) -> u32 {
        self.items[i as usize].1
    }

    /// This list is empty or not
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Append item
    pub fn append(&mut self, item: Item, n: u32) {
        
        if self.items.is_empty() {
            self.items.push((item, n));
            return;
        }

        for i in 0..self.items.len() {
            match item.cmp(&self.items[i].0) { 
                Ordering::Equal => { // If this list has the same item, increases the number
                    self.items[i].1 += n;
                    return;
                }
                Ordering::Less => {
                    self.items.insert(i, (item, n));
                    return;
                }
                Ordering::Greater => {
                    continue;
                }
            }
        }
        self.items.push((item, n));
    }

    /// Remove an item from list
    pub fn remove<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);
        if n == 0 { return; }

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i);
        }
    }

    /// Remove an item from list and get its clone or moved value
    pub fn remove_and_get<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) -> Item {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        }
    }

    /// Move an item to the other item list
    pub fn move_to<T: Into<ItemMoveNum>>(&mut self, dest: &mut ItemList, i: usize, n: T) {
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);
        
        self.items[i].1 -= n;

        let item = if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        };

        dest.append(item, n);
    }

    /// Clear all item in list
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Return item iterator
    pub fn iter(&self) -> ::std::slice::Iter<(Item, u32)> {
        self.items.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemMoveNum {
    All, Partial(u32),
}

impl ItemMoveNum {
    fn to_u32(self, all: u32) -> u32 {
        match self {
            ItemMoveNum::All => all,
            ItemMoveNum::Partial(n) => n,
        }
    }
}

impl From<u32> for ItemMoveNum {
    fn from(n: u32) -> ItemMoveNum {
        ItemMoveNum::Partial(n)
    }
}

//
// Equipment handling types and routines
//

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum EquipSlotKind {
    MeleeWeapon, RangedWeapon, BodyArmor, Shield,
}

impl ItemKind {
    pub fn equip_slot_kind(self) -> Option<EquipSlotKind> {
        match self {
            ItemKind::Weapon(weapon_kind) => Some(weapon_kind.equip_slot_kind()),
            ItemKind::Armor(armor_kind) => Some(armor_kind.equip_slot_kind()),
            _ => None
        }
    }
}

impl WeaponKind {
    pub fn equip_slot_kind(self) -> EquipSlotKind {
        use self::WeaponKind::*;
        match self {
            Axe | Spear | Sword => EquipSlotKind::MeleeWeapon,
            _ => EquipSlotKind::RangedWeapon,
        }
    }
}

impl ArmorKind {
    pub fn equip_slot_kind(self) -> EquipSlotKind {
        match self {
            ArmorKind::Body => EquipSlotKind::BodyArmor,
            ArmorKind::Shield => EquipSlotKind::Shield,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EquipItemList {
    /// Slot infomation
    slots: Vec<SlotInfo>,
    item_list: ItemList,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SlotInfo {
    /// The kind of equipment
    esk: EquipSlotKind,
    /// The index in this ItemKind
    n: u8,
    /// The Index at list
    list_idx: Option<u8>,
}

impl SlotInfo {
    fn new(esk: EquipSlotKind, n: u8) -> SlotInfo {
        SlotInfo { esk, n, list_idx: None }
    }
}

pub const MAX_SLOT_NUM_PER_KIND: usize = ::basic::MAX_EQUIP_SLOT as usize;

impl EquipItemList {
    pub fn new(slots: &[(EquipSlotKind, u8)]) -> EquipItemList {
        let mut slots = slots.to_vec();
        slots.sort_by_key(|&(ik, _)| ik);
        let mut new_slots = Vec::new();
        for &(esk, n) in slots.iter() {
            for i in 0..n {
                new_slots.push(SlotInfo::new(esk, i));
            }
        }
        
        EquipItemList {
            slots: new_slots,
            item_list: ItemList::new(),
        }
    }

    /// Number of slots for specified ItemKind
    pub fn slot_num(&self, esk: EquipSlotKind) -> usize {
        self.slots.iter().filter(|slot| slot.esk == esk).count()
    }
    
    /// Specified slot is empty or not
    /// If specified slot doesn't exist, return false.
    pub fn is_slot_empty(&self, esk: EquipSlotKind, n: usize) -> bool {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        if let Some(a) = self.slots.iter().filter(|slot| slot.esk == esk).nth(n) {
            a.list_idx.is_none()
        } else {
            false
        }
    }

    /// Get specified equipped item
    pub fn item(&self, esk: EquipSlotKind, n: usize) -> Option<&Item> {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        if let Some(a) = self.list_idx(esk, n) {
            Some(&self.item_list.items[a].0)
        } else {
            None
        }
    }
    
    /// Equip an item to specified slot (the nth slot of given ItemKind), and returns removed item
    pub fn equip(&mut self, esk: EquipSlotKind, n: usize, item: Item) -> Option<Item> {
        assert!(self.slot_num(esk) > n);
        if let Some(i) = self.list_idx(esk, n) { // Replace existing item
            return Some(::std::mem::replace(&mut self.item_list.items[i].0, item));
        }
        
        if self.item_list.items.is_empty() { // If any item is not equipped.
            self.item_list.items.push((item, 1));
            self.set_list_idx(esk, n, 0);
            return None;
        }
        
        // Calculate new index for insert
        let mut new_idx = 0;
        let mut processed_slot = 0;
        for i_slot in 0..self.slots.len() {
            if self.slots[i_slot].esk == esk && self.slots[i_slot].n as usize == n {
                self.set_list_idx(esk, n, new_idx);
                self.item_list.items.insert(new_idx, (item, 1));
                processed_slot = i_slot;
                break;
            } else if self.slots[i_slot].list_idx.is_some() {
                new_idx += 1;
            }
        }

        for i_slot in (processed_slot + 1)..self.slots.len() {
            if let Some(list_idx) = self.slots[i_slot].list_idx {
                self.slots[i_slot].list_idx = Some(list_idx + 1);
            }
        }
        
        None
    }

    fn list_idx(&self, esk: EquipSlotKind, n: usize) -> Option<usize> {
        if let Some(slot) = self.slots.iter().find(|slot| slot.esk == esk && slot.n as usize == n) {
            if let Some(list_idx) = slot.list_idx {
                Some(list_idx as usize)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_list_idx(&mut self, esk: EquipSlotKind, n: usize, idx: usize) {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.esk == esk && slot.n as usize == n) {
            slot.list_idx = Some(idx as u8);
        } else {
            panic!("set_list_idx for invalid slot");
        }
    }

    pub fn slot_iter(&self) -> EquipSlotIter {
        EquipSlotIter {
            equip_item_list: &self,
            n: 0,
        }
    }

    pub fn item_iter(&self) -> EquipItemIter {
        EquipItemIter {
            equip_item_list: &self,
            n: 0,
        }
    }

    pub fn list(&self) -> &ItemList {
        &self.item_list
    }

    pub fn n_slots(&self) -> u32 {
        self.slots.len() as u32
    }
}

pub struct EquipSlotIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipSlotIter<'a> {
    type Item = (EquipSlotKind, u8, Option<&'a Item>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.equip_item_list.slots.len() {
            return None;
        }
        let slot = &self.equip_item_list.slots[self.n];
        let result = if let Some(i) = slot.list_idx {
            (slot.esk, slot.n, Some(&self.equip_item_list.item_list.items[i as usize].0))
        } else {
            (slot.esk, slot.n, None)
        };
        self.n += 1;
        return Some(result);
    }
}
    
pub struct EquipItemIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipItemIter<'a> {
    type Item = (EquipSlotKind, u8, &'a Item);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n >= self.equip_item_list.slots.len() {
                return None;
            }
            let slot = &self.equip_item_list.slots[self.n];
            if let Some(i) = slot.list_idx {
                let result = (slot.esk, slot.n, &self.equip_item_list.item_list.items[i as usize].0);
                self.n += 1;
                return Some(result);
            }
            self.n += 1;
        }
    }
}

// Implement serialize & deserialize for ItemFlags
mod impl_serde {
    use serde::ser::{Serialize, Serializer};
    use serde::de::{Deserialize, Deserializer, Visitor};
    use std::fmt;
    use super::ItemFlags;
    
    impl Serialize for ItemFlags {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let bits = self.bits();
            serializer.serialize_u64(bits)
        }
    }

    struct ItemFlagsVisitor;

    impl<'de> Visitor<'de> for ItemFlagsVisitor {
        type Value = ItemFlags;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<ItemFlags, E> where E: ::serde::de::Error {
            Ok(ItemFlags::from_bits_truncate(v))
        }
    }

    impl<'de> Deserialize<'de> for ItemFlags {
        fn deserialize<D>(deserializer: D) -> Result<ItemFlags, D::Error>
        where D: Deserializer<'de>
        {
            deserializer.deserialize_u64(ItemFlagsVisitor)
        }
    }
}

