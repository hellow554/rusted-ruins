
use array2d::*;
use common::basic::MAX_ITEM_FOR_DRAW;
use common::objholder::*;
use common::gamedata::*;
use common::gobj;
use common::obj::SpecialTileObject;
use game::view::ViewMap;

/// Needed infomation to draw background parts of an tile
/// "Background" means that they are drawed behind any characters
#[derive(Default)]
pub struct BackgroundDrawInfo {
    pub tile: Option<TileLayers>,
    pub special: Option<SpecialTileIdx>,
}

impl BackgroundDrawInfo {
    pub fn new(map: &Map, pos: Vec2d) -> BackgroundDrawInfo {
        let mut di = BackgroundDrawInfo::default();
        
        let tile = if map.is_inside(pos) {
            let tinfo = &map.observed_tile[pos];
            
            tinfo.tile
        } else {
            if let Some(ref outside_tile) = map.outside_tile {
                Some(outside_tile.tile.into())
            } else {
                let pos = map.nearest_existent_tile(pos);
                let tinfo = &map.observed_tile[pos];
                tinfo.tile
            }
        };
        
        di.tile = tile;

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.observed_tile[pos].special.obj_id() {
                let special_tile_obj: &'static SpecialTileObject = gobj::get_by_id(special_tile_id);
                if special_tile_obj.always_background {
                    let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id);
                    di.special = Some(special_tile_idx);
                }
            }
        }
        
        di
    }
}

/// Needed infomation to draw foreground parts of an tile
/// "Foreground" means that they are drawed infront characters
/// whose are on the prev row
#[derive(Default)]
pub struct ForegroundDrawInfo {
    pub special: Option<SpecialTileIdx>,
    pub wallpp: WallIdxPP,
    pub deco: Option<DecoIdx>,
    pub n_item: usize,
    pub items: [ItemIdx; MAX_ITEM_FOR_DRAW],
    pub chara: Option<CharaId>,
}

impl ForegroundDrawInfo {
    pub fn new(map: &Map, view_map: &ViewMap, pos: Vec2d) -> ForegroundDrawInfo {
        let mut di = ForegroundDrawInfo::default();

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.observed_tile[pos].special.obj_id() {
                let special_tile_obj: &'static SpecialTileObject = gobj::get_by_id(special_tile_id);
                if !special_tile_obj.always_background {
                    let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id);
                    di.special = Some(special_tile_idx);
                }
            }
        }

        di.wallpp = if map.is_inside(pos) {
            di.deco = map.observed_tile[pos].deco;
            map.observed_tile[pos].wall
        } else {
            if let Some(ref outside_tile) = map.outside_tile {
                if let Some(wall_idx) = outside_tile.wall {
                    WallIdxPP {
                        idx: wall_idx,
                        piece_pattern: PiecePattern::SURROUNDED,
                    }
                } else {
                    WallIdxPP::default()
                }
            } else {
                let nearest_pos = map.nearest_existent_tile(pos);
                let mut wallpp = map.observed_tile[nearest_pos].wall;
                if !wallpp.is_empty() {
                    adjust_pattern_from_nearest(&mut wallpp.piece_pattern, pos, nearest_pos);
                }
                wallpp
            }
        };

        if view_map.get_tile_visible(pos) {
            di.chara = map.get_chara(pos);
        }

        // Set items
        if map.is_inside(pos) {
            let tinfo = &map.observed_tile[pos];
            let n_item = tinfo.n_item;
            for i in 0..n_item {
                di.items[i] = tinfo.items[i];
            }
            di.n_item = n_item;
        }
        
        di
    }

}

/// Adjust piece pattern when getting piece pattern from the nearest tile.
fn adjust_pattern_from_nearest(pp: &mut PiecePattern, _pos: Vec2d, _nearest_pos: Vec2d) {
    *pp = PiecePattern::SURROUNDED;
}

