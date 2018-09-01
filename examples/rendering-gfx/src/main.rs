#[macro_use]
extern crate spiner;
extern crate image;
extern crate libc;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate libspine_sys;
extern crate rand;

mod run;

use gfx::traits::FactoryExt;
use gfx::{Device, Factory};
use gfx_window_glutin as gfx_glutin;
use glutin::GlContext;

use rand::Rng;

use std::fs::{File, self};
use std::io::Error;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::slice::Iter;

use spiner::animation::state::{State as AnimationState, StateData};
use spiner::atlas::{page::Page, Atlas};
use spiner::attachment::vertex::Vertex as VertexAttachment;
use spiner::attachment::Attachment;
use spiner::extension::Texture;
use spiner::skeleton::data::Data as SkeletonData;
use spiner::skeleton::json::Json as SkeletonJson;
use spiner::skeleton::Skeleton;

const MAX_VERTICES: usize = 1000;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        position: [0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "position",
        tex_coords: [f32; 2] = "tex_coords",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        perspective: gfx::Global<[[f32; 3]; 3]> = "perspective",
        texture: gfx::TextureSampler<[f32; 4]> = "tex",
        out_color: gfx::BlendTarget<ColorFormat> = ("color", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

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

            path.components()
                .last()
                .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
        })
        .filter_map(|ref name| Asset::load(name, "./assets").ok())
        .collect();

    let mut world_vertices = vec![0.; MAX_VERTICES];

    // setup glium
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions((800, 600).into())
        .with_title("Spiner rendering example".to_owned());
    let context_builder = glutin::ContextBuilder::new();
    let (window, mut device, mut factory, color_view, depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(
            include_bytes!("../gl/spine.vert"),
            include_bytes!("../gl/spine.frag"),
            pipe::new(),
        )
        .unwrap();


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
            let kind =
                gfx::texture::Kind::D2(texture.width as u16, texture.height as u16, gfx::texture::AaMode::Single);
            let (_, view) = factory
                .create_texture_immutable_u8::<ColorFormat>(kind, gfx::texture::Mipmap::Provided, &[&texture.buffer])
                .unwrap();

            textures.insert(page.name.clone(), view);
        }
    }

    let (quad_vertex_buffer, _) =
        factory.create_vertex_buffer_with_slice(&QUAD_VERTS, &QUAD_INDICES[..]);

    let sampler = factory.create_sampler_linear();
    let mut data = pipe::Data {
        vbuf: quad_vertex_buffer,
        perspective: [[0.; 3]; 3],
        texture: (textures.values().next().unwrap().clone(), sampler),
        out_color: color_view
    };

    run::start_loop((1_000_000_000.0 / 60.) as u64, || {
        let (width, height): (u32, u32) = window.get_inner_size().unwrap().into();
        data.perspective[0][0] = 1. / width as f32;
        data.perspective[1][1] = 1. / height as f32;
        data.perspective[2][2] = 1.;

        encoder.clear(&data.out_color, BLACK);

        assets.iter_mut().for_each(|asset| asset.update(0.01));

        for asset in assets.iter() {
            let (page, vertices, indices) = asset.batch(&mut world_vertices);

            if let Some(texture) = page.and_then(|p| textures.get(&p.name)) {
                let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &*indices);
                data.vbuf = vertex_buffer;
                data.texture.0 = texture.clone();

                encoder.draw(&slice, &pso, &data);
            }
        }

        let mut action = run::Action::Continue;
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => action = run::Action::Stop,
                _ => (),
            },
            _ => (),
        });

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

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

                (QUAD_INDICES[..].to_vec(), region.uvs().to_vec())
            }
            _ => continue,
        };
        let (width, height) = page.as_ref()
            .map(|p| (p.width as f32, p.height as f32))
            .unwrap_or((1., 1.));
        let to_tex_coords = |x: f32, y: f32| [x / width, y / height];

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
