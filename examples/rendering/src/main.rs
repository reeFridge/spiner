#[macro_use]
extern crate spiner;
extern crate libc;
extern crate png;
#[macro_use]
extern crate glium;
extern crate libspine_sys;

mod run;

use glium::index::PrimitiveType;
use glium::Surface;

use std::fs::File;
use std::io::Error;
use std::io::Read;

use spiner::animation::state::{State as AnimationState, StateData};
use spiner::atlas::Atlas;
use spiner::attachment::Attachment;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::Skeleton;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    //  tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position);

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
    // setup glium
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions((800, 600).into())
        .with_title("Spiner rendering example".to_owned());
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex_src = include_str!("../gl/spine.vert");
    let fragment_src = include_str!("../gl/spine.frag");
    let program = glium::Program::from_source(&display, vertex_src, fragment_src, None).unwrap();

    // setup spine
    let mut atlas =
        Atlas::from_file("./assets/spineboy/spineboy.atlas").expect("Cannot read atlas");
    let skeleton_data = SkeletonJson::new(&mut atlas, 1.)
        .read_skeleton_file("./assets/spineboy/spineboy.json")
        .expect("Cannot parse skeleton data");

    let mut animation_state_data = StateData::from(&skeleton_data);
    animation_state_data.set_default_mix(0.5);

    let mut animation_state = AnimationState::from(&animation_state_data);
    let mut skeleton = Skeleton::from(&skeleton_data);

    skeleton.set_position((0., -300.));

    let animations = skeleton_data.animations();
    animations.iter().enumerate().for_each(|(i, anim)| {
        println!("#{}: {}", i, anim.name);
    });

    // Choose animation to play
    let anim = animations.iter().nth(4);
    animation_state.set_animation(0, anim.unwrap(), true);

    let mut perspective = [[0.0; 3]; 3];

    // the main loop
    run::start_loop((1_000_000_000.0 / 60.) as u64, || {
        let mut target = display.draw();

        let (width, height) = target.get_dimensions();
        perspective[0][0] = 1.0 / width as f32;
        perspective[1][1] = 1.0 / height as f32;
        perspective[2][2] = 1.0;

        target.clear_color(0.0, 0.0, 1.0, 0.0);

        animation_state.update(0.01);
        animation_state.apply(&mut skeleton);
        skeleton.update_world_transform();
        
        let (vertices, indices) = compute_skeleton_vertices(&skeleton);
        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
        let index_buffer =
            glium::index::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)
                .unwrap();
        let uniforms = uniform! {
            perspective: perspective
        };
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        let mut action = run::Action::Continue;
        events_loop.poll_events(|event| match event {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                glium::glutin::WindowEvent::CloseRequested => action = run::Action::Stop,
                _ => (),
            },
            _ => (),
        });

        action
    });
}

fn compute_skeleton_vertices(skeleton: &Skeleton) -> (Vec<Vertex>, Vec<u32>) {
    let mut pos = vec![0.; 8];
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut n = 0 as usize;
    for slot in skeleton.slots_ordered().iter() {
        let attachment = match slot.attachment() {
            None => continue,
            Some(attach) => attach,
        };

        let mut push_vertex = |x: f32, y: f32| {
            vertices.push(Vertex { position: [x, y] });
        };

        match attachment {
            Attachment::Region(region) => {
                region.compute_world_vertices(&slot.bone().unwrap(), &mut pos, 0, 2);
                push_vertex(pos[0], pos[1]);
                push_vertex(pos[2], pos[3]);
                push_vertex(pos[4], pos[5]);
                push_vertex(pos[6], pos[7]);

                let i = 4 * n as u32;
                indices.push(i);
                indices.push(i + 1);
                indices.push(i + 2);
                indices.push(i + 2);
                indices.push(i + 3);
                indices.push(i);

                n += 1;
            }
            _ => continue,
        }
    }

    (vertices, indices)
}
