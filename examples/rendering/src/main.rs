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
use spiner::attachment::vertex::Vertex as VertexAttachment;
use spiner::attachment::Attachment;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::Skeleton;

const MAX_VERTICES: usize = 1000;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
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
    let mut atlas = Atlas::from_file("./assets/raptor/raptor.atlas").expect("Cannot read atlas");
    let skeleton_data = SkeletonJson::new(&mut atlas, 1.)
        .read_skeleton_file("./assets/raptor/raptor.json")
        .expect("Cannot parse skeleton data");

    let mut animation_state_data = StateData::from(&skeleton_data);
    animation_state_data.set_default_mix(0.5);

    let mut animation_state = AnimationState::from(&animation_state_data);
    let mut skeleton = Skeleton::from(&skeleton_data);

    skeleton.set_position((0., -500.));

    let animations = skeleton_data.animations();
    animations.iter().enumerate().for_each(|(i, anim)| {
        println!("#{}: {}", i, anim.name);
    });

    // Choose animation to play
    let anim = animations.iter().nth(3);
    animation_state.set_animation(0, anim.unwrap(), true);

    let mut perspective = [[0.; 3]; 3];
    let mut world_vertices = vec![0.; MAX_VERTICES];

    // the main loop
    run::start_loop((1_000_000_000.0 / 60.) as u64, || {
        let mut target = display.draw();

        let (width, height) = target.get_dimensions();
        perspective[0][0] = 1. / width as f32;
        perspective[1][1] = 1. / height as f32;
        perspective[2][2] = 1.;

        target.clear_color(0., 0., 1., 0.);

        animation_state.update(0.01);
        animation_state.apply(&mut skeleton);
        skeleton.update_world_transform();

        let (vertices, indices) = compute_skeleton_vertices(&skeleton, &mut world_vertices);
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

fn compute_skeleton_vertices(
    skeleton: &Skeleton,
    world_vertices: &mut Vec<f32>,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::<u32>::new();
    let quad_indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    for slot in skeleton.slots_ordered().iter() {
        let attachment = match slot.attachment() {
            None => continue,
            Some(attach) => attach,
        };

        let (attachment_indices, uvs) = match attachment {
            Attachment::Mesh(mesh) => {
                let len = mesh.world_vertices_len();
                mesh.compute_world_vertices(&slot, 0, len as i32, world_vertices, 0, 2);

                (mesh.triangles(), mesh.uvs())
            }
            Attachment::Region(region) => {
                region.compute_world_vertices(&slot.bone().unwrap(), world_vertices, 0, 2);

                (quad_indices.to_vec(), region.uvs().to_vec())
            }
            _ => continue,
        };

        for index in attachment_indices.iter() {
            // multiply by two (use bitwice left-shift cause u16)
            let index = (*index << 1) as usize;

            vertices.push(Vertex {
                position: [world_vertices[index], world_vertices[index + 1]],
                tex_coords: [uvs[index], uvs[index + 1]],
            });
            indices.push((vertices.len() - 1) as u32);
        }
    }

    (vertices, indices)
}
