#[macro_use]
extern crate spiner;
extern crate png;
extern crate libc;

use std::io::Error;
use std::fs::File;
use std::io::Read;

use spiner::atlas::Atlas;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::Skeleton;
use spiner::attachment::Base;
use spiner::animation::state::{StateData, State as AnimationState};
use spiner::attachment::Attachment;

fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    println!("read file {}", path);
    let mut buf = Vec::new();
    File::open(path).and_then(|mut f| f.read_to_end(&mut buf))?;

    Ok(buf)
}

fn read_texture(path: &str) -> Result<(Vec<u8>, (u32, u32)), Error> {
    println!("read texture {}", path);
    let decoder = png::Decoder::new(File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let len = info.buffer_size();
    let mut buf = vec![0; len];
    reader.next_frame(&mut buf)?;

    Ok((buf, (info.width, info.height)))
}

extend_spine!({
    _spUtil_readFile -> read_file,
    _spAtlasPage_createTexture -> read_texture
});

fn main() {
    let mut atlas = Atlas::from_file("./assets/raptor/raptor.atlas")
        .expect("Cannot read atlas");
    let skeleton_data = SkeletonJson::new(&mut atlas, 2.)
        .read_skeleton_file("./assets/raptor/raptor.json")
        .expect("Cannot parse skeleton data");

    let mut animation_state_data = StateData::from(&skeleton_data);
    animation_state_data.set_default_mix(0.5);

    let mut animation_state = AnimationState::from(&animation_state_data);

    let mut skeleton = Skeleton::from(&skeleton_data);

    skeleton.set_position((200., 200.));
    skeleton_data.animations().iter().enumerate().for_each(|(i, anim)| {
        animation_state.set_animation(i as i32, anim, false);
    });

    for i in 0..10 {
        println!("loop cycle #{}", i);
        animation_state.update(0.1);
        animation_state.apply(&mut skeleton);
        skeleton.update_world_transform();
        draw_skeleton(&skeleton);
    }
}

fn draw_skeleton(skeleton: &Skeleton) {
    for slot in skeleton.slots_ordered().iter() {
        let attachment = match slot.attachment() {
            None => continue,
            Some(attach) => attach
        };

        match attachment {
            Attachment::Mesh(mesh) => {}
            Attachment::Region(region) => {}
            _ => continue
        }
    }
}
