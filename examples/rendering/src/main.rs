#[macro_use]
extern crate spiner;
extern crate image;
extern crate libc;
#[macro_use]
extern crate glium;
extern crate libspine_sys;

mod run;

use glium::index::PrimitiveType;
use glium::Surface;

use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Read;
use std::rc::Rc;

use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use spiner::animation::state::{State as AnimationState, StateData};
use spiner::atlas::{page::Page, Atlas};
use spiner::attachment::vertex::Vertex as VertexAttachment;
use spiner::attachment::Attachment;
use spiner::extension::Texture;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::Skeleton;

const MAX_VERTICES: usize = 1000;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    println!("read file {}", path);
    let mut buf = Vec::new();
    File::open(path).and_then(|mut f| f.read_to_end(&mut buf))?;

    Ok(buf)
}

fn read_texture(path: &str) -> Result<Texture, Error> {
    println!("read texture {}", path);
    let image = image::open(path).unwrap().to_rgba();
    let (width, height) = image.dimensions();

    Ok(Texture {
        buffer: image.into_raw(),
        width: width,
        height: height,
    })
}

extend_spine!({
    _spUtil_readFile -> read_file,
    _spAtlasPage_createTexture -> read_texture
});

struct Asset {
    pub name: String,
    path: String,
}

impl Asset {
    pub fn new(name: &str, path: &str) -> Self {
        Asset {
            name: name.to_string(),
            path: path.to_string(),
        }
    }

    pub fn atlas(&self) -> String {
        format!("{}/{}/{}.atlas", self.path, self.name, self.name)
    }

    pub fn skeleton(&self) -> String {
        format!("{}/{}/{}.json", self.path, self.name, self.name)
    }
}

fn main() {
    let asset = Asset::new("raptor", "./assets");

    // setup spine
    let mut atlas = Atlas::from_file(&asset.atlas()).expect("Cannot read atlas");

    // Fetch pages with textures for preloading
    let pages: Vec<Page> = atlas.pages();

    // Read, parse and store all skeleton information
    let skeleton_data = Rc::new(
        SkeletonJson::new(&mut atlas, 1.)
            .expect("Cannot create skeleton json reader")
            .read_skeleton_file(&asset.skeleton())
            .expect("Cannot parse skeleton data"),
    );

    // Share part of skeleton info for animating
    let mut animation_state_data = Rc::new(
        StateData::from_skeleton_data(Rc::clone(&skeleton_data))
            .expect("Cannot create animation state data"),
    );
    Rc::get_mut(&mut animation_state_data)
        .unwrap()
        .set_default_mix(0.5);

    // One animation data may be used by several animation states
    let mut animation_state = AnimationState::from_data(Rc::clone(&animation_state_data))
        .expect("Cannot create animation state");

    // Choose animation to play
    let animations = skeleton_data.animations();
    animations.iter().enumerate().for_each(|(i, anim)| {
        println!("#{}: {}", i, anim.name);
    });

    println!("Enter animation name/number:");
    let mut input = String::new();
    let animation = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            if let Ok(num) = input.parse::<u32>() {
                animations.iter().nth(num as usize).cloned()
            } else if !input.is_empty() {
                skeleton_data.find_animation_by_name(input)
            } else {
                panic!("You must enter animation name or number");
            }
        }
        Err(error) => panic!("error: {}", error),
    };

    match animation {
        Some(ref animation) => animation_state.set_animation(0, animation, true),
        None => panic!("animation not found"),
    }

    let mut skeleton =
        Skeleton::from_data(Rc::clone(&skeleton_data)).expect("Cannot create skeleton");
    skeleton.set_position((0., -300.));
    let mut perspective = [[0.; 3]; 3];
    let mut world_vertices = vec![0.; MAX_VERTICES];

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
    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    // Preload textures
    let mut textures = std::collections::HashMap::new();
    let pages_iter = pages.iter().filter(|page| page.renderer_object().is_some());

    for page in pages_iter {
        println!("load page {} into texture", page.name);
        let texture = page.renderer_object().unwrap();
        let image =
            RawImage2d::from_raw_rgba_reversed(&texture.buffer, (texture.width, texture.height));

        textures.insert(
            page.name.clone(),
            CompressedSrgbTexture2d::new(&display, image).unwrap(),
        );
    }

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

        let (page, vertices, indices) = compute_skeleton_vertices(&skeleton, &mut world_vertices);

        if let Some(texture) = page.and_then(|p| textures.get(&p.name)) {
            let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
            let index_buffer =
                glium::index::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)
                    .unwrap();
            let uniforms = uniform! {
                perspective: perspective,
                tex: texture
            };
            target
                .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
                .unwrap();
            target.finish().unwrap();
        }

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
) -> (Option<Page>, Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::<u32>::new();
    let quad_indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
    let mut page = None;

    for slot in skeleton.slots_ordered().iter_mut() {
        let attachment = match slot.attachment() {
            None => continue,
            Some(attach) => attach,
        };

        let (attachment_indices, uvs) = match attachment {
            Attachment::Mesh(mesh) => {
                let len = mesh.world_vertices_len();
                mesh.compute_world_vertices(slot, 0, len as i32, world_vertices, 0, 2);
                if page.as_ref().is_none() {
                    page = mesh.atlas_region().and_then(|region| region.page());
                }

                (mesh.triangles(), mesh.uvs())
            }
            Attachment::Region(mut region) => {
                region.compute_world_vertices(&mut slot.bone().unwrap(), world_vertices, 0, 2);
                if page.as_ref().is_none() {
                    page = region.atlas_region().and_then(|region| region.page());
                }

                (quad_indices.to_vec(), region.uvs().to_vec())
            }
            _ => continue,
        };
        let (width, height) = page
            .as_ref()
            .map(|p| (p.width as f32, p.height as f32))
            .unwrap_or((1., 1.));
        let to_tex_coords = |x: f32, y: f32| [x / width, 1.0 - y / height];

        for index in attachment_indices.iter() {
            // multiply by two (use bitwice left-shift cause u16)
            let index = (*index << 1) as usize;

            vertices.push(Vertex {
                position: [world_vertices[index], world_vertices[index + 1]],
                tex_coords: to_tex_coords(uvs[index] * width, uvs[index + 1] * height),
            });
            indices.push((vertices.len() - 1) as u32);
        }
    }

    (page, vertices, indices)
}
