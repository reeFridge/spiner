#[macro_use]
extern crate spiner;
extern crate image;
extern crate libc;
#[macro_use]
extern crate glium;
extern crate libspine_sys;
extern crate rand;

mod run;

use rand::Rng;

use glium::index::PrimitiveType;
use glium::Surface;

use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Read;
use std::rc::Rc;
use std::slice::Iter;
use std::path::Path;
use std::fs::{self, DirEntry};

use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use spiner::animation::state::{State as AnimationState, StateData};
use spiner::atlas::{page::Page, Atlas};
use spiner::attachment::vertex::Vertex as VertexAttachment;
use spiner::attachment::Attachment;
use spiner::extension::Texture;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::data::Data as SkeletonData;
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
    atlas: Atlas,
    skeleton_data: Rc<SkeletonData>,
    state_data: Rc<StateData>,
    skeleton: Skeleton,
    animation_state: AnimationState,
}

impl Asset {
    pub fn load(name: &str, path: &str) -> Result<Self, Error> {
        let atlas = Atlas::from_file(&Asset::atlas_path(name, path))?;
        let skeleton_data = Rc::new(
            SkeletonJson::new(&atlas, 1.)?.read_skeleton_file(&Asset::skeleton_path(name, path))?
        );
        let mut state_data = StateData::from_skeleton_data(Rc::clone(&skeleton_data))?;
        state_data.set_default_mix(0.5);
        let state_data = Rc::new(state_data);

        let animations = skeleton_data.animations();
        let len = animations.len();

        let mut animation_state = AnimationState::from_data(Rc::clone(&state_data))?;
        animation_state.set_animation(
            0,
            animations
                .iter()
                .nth(rand::thread_rng().gen_range(len - 1, len) as usize)
                .unwrap(),
            true,
        );

        let mut skeleton = Skeleton::from_data(Rc::clone(&skeleton_data))?;
        skeleton.set_position((0., -500.));

        Ok(Asset {
            name: name.to_string(),
            atlas,
            skeleton_data,
            state_data,
            animation_state,
            skeleton,
        })
    }

    pub fn update(&mut self, delta: f32) {
        self.animation_state.update(delta);
        self.animation_state.apply(&mut self.skeleton);
        self.skeleton.update_world_transform();
    }

    pub fn batch(&self, world_vertices: &mut Vec<f32>) -> (Option<Page>, Vec<Vertex>, Vec<u32>) {
        batch_skeleton_vertices(&self.skeleton, world_vertices)
    }

    pub fn resources(&self) -> Iter<Page> {
        self.atlas.pages()
    }

    fn atlas_path(name: &str, path: &str) -> String {
        format!("{}/{}/{}.atlas", path, name, name)
    }

    fn skeleton_path(name: &str, path: &str) -> String {
        format!("{}/{}/{}.json", path, name, name)
    }
}

fn main() -> std::io::Result<()> {
    let assets_dir = Path::new("./assets");
    let mut assets: Vec<Asset> = fs::read_dir(assets_dir)?
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                return None;
            }

            path.components().last().map(|comp| comp.as_os_str().to_string_lossy().into_owned())
        })
        .filter_map(|ref name| Asset::load(name, "./assets").ok())
        .collect();

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
    {
        let pages_iter = assets
            .iter()
            .flat_map(|asset| asset.resources())
            .filter(|page| page.renderer_object().is_some());

        for page in pages_iter {
            println!("load page {} into texture", page.name);
            let texture = page.renderer_object().unwrap();
            let image = RawImage2d::from_raw_rgba_reversed(
                &texture.buffer,
                (texture.width, texture.height),
            );

            textures.insert(
                page.name.clone(),
                CompressedSrgbTexture2d::new(&display, image).unwrap(),
            );
        }
    }

    run::start_loop((1_000_000_000.0 / 60.) as u64, || {
        let mut target = display.draw();

        let (width, height) = target.get_dimensions();
        perspective[0][0] = 1. / width as f32;
        perspective[1][1] = 1. / height as f32;
        perspective[2][2] = 1.;

        target.clear_color(0., 0., 1., 0.);

        assets.iter_mut().for_each(|asset| asset.update(0.01));

        for asset in assets.iter() {
            let (page, vertices, indices) = asset.batch(&mut world_vertices);

            if let Some(texture) = page.and_then(|p| textures.get(&p.name)) {
                let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
                let index_buffer = glium::index::IndexBuffer::new(
                    &display,
                    PrimitiveType::TrianglesList,
                    &indices,
                ).unwrap();
                let uniforms = uniform! {
                    perspective: perspective,
                    tex: texture
                };
                target
                    .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
                    .unwrap();
            }
        }

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

    Ok(())
}

fn batch_skeleton_vertices(
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
        let (width, height) = page.as_ref()
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
