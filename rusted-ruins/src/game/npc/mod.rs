//! Functions for NPC's AI and actions

pub mod map_search;

use array2d::*;
use common::gamedata::*;
use super::{Game, InfoGetter};
use super::action;
use rng::*;

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {

    {
        let chara = game.gd.chara.get(cid);
        let ai = &chara.ai;

        match ai.kind {
            NpcAIKind::None => {
                return;
            }
            NpcAIKind::NoMove => {
                return;
            }
            _ => (),
        }
    }

    if gen_range(0, 3) == 0 {
        move_to_nearest_enemy(game, cid);
    } else {
        random_walk(game, cid);
    }
}

/// Move npc at random
fn random_walk(game: &mut Game, cid: CharaId) {
    let dir = Direction::new(
        *get_rng().choose(&[HDirection::Left, HDirection::None, HDirection::Right]).unwrap(),
        *get_rng().choose(&[VDirection::Up, VDirection::None, VDirection::Down]).unwrap());
    action::try_move(game, cid, dir);
}

/// Move npc to nearest enemy
fn move_to_nearest_enemy(game: &mut Game, cid: CharaId) {
    if let Some(target) = map_search::search_nearest_enemy(&game.gd, cid) {
        if let Some(pos) = game.gd.chara_pos(cid) {
            let dir = map_search::dir_to_chara(&game.gd, target, pos);
            action::try_move(game, cid, dir);
        }
    }
}

