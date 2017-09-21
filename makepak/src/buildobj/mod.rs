
mod img;
mod item;

use common::obj::*;
use tomlinput::TomlInput;
use error::*;
use self::img::*;
use self::item::build_item_object;

pub fn build_object(tomlinput: TomlInput) -> Result<Object> {
    let object_type = tomlinput.object_type.clone();
    match object_type.as_ref() {
        "tile" => {
            return build_tile_object(tomlinput).map(|o| Object::Tile(o));
        }
        "wall" => {
            return build_wall_object(tomlinput).map(|o| Object::Wall(o));
        }
        "item" => {
            return build_item_object(tomlinput).map(|o| Object::Item(o));
        }
        "chara_template" => {
            return build_chara_template_object(tomlinput).map(|o| Object::CharaTemplate(o));
        }
        "anim_img" => {
            return build_anim_img_object(tomlinput).map(|o| Object::AnimImg(o));
        }
        _ => {
            bail!("Unknown object_type");
        }
    }
}

fn build_tile_object(tomlinput: TomlInput) -> Result<TileObject> {
    let _tile_dep_input = get_optional_field!(tomlinput, tile);
    let img = get_optional_field!(tomlinput, image);
    
    Ok(TileObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_wall_object(tomlinput: TomlInput) -> Result<WallObject> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(WallObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_chara_template_object(tomlinput: TomlInput) -> Result<CharaTemplateObject> {
    let chara_dep_input = get_optional_field!(tomlinput, chara_template);
    let img = get_optional_field!(tomlinput, image);
    
    Ok(CharaTemplateObject {
        id: tomlinput.id,
        img: build_img(img)?,
        str: chara_dep_input.str,
        vit: chara_dep_input.vit,
        dex: chara_dep_input.dex,
        int: chara_dep_input.int,
        wil: chara_dep_input.wil,
        cha: chara_dep_input.cha,
        spd: chara_dep_input.spd,
    })
}

fn build_anim_img_object(tomlinput: TomlInput) -> Result<AnimImgObject> {
    let anim_img_dep = get_optional_field!(tomlinput, anim_img);
    let img = get_optional_field!(tomlinput, image);

    Ok(AnimImgObject {
        id: tomlinput.id,
        img: build_img(img)?,
        duration: anim_img_dep.duration,
        n_frame: anim_img_dep.n_frame,
    })
}

