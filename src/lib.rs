extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use cgmath::SquareMatrix;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::texture;
use glutin::GlContext;
use gfx::format::{Depth, Rgba8};
use image::{GenericImage, RgbaImage};
use gfx::Factory;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


gfx_defines! {
    vertex Vertex {
        position: [f32; 3] = "position",
        tex_coord: [f32; 3] = "tex_coord",
        color: [f32; 4] = "color",
        normal: [f32; 3] = "normal",
    }

    constant Locals {
        u_transform: [[f32; 4]; 4] = "u_transform",
        u_color: [f32; 4] = "u_color",
        u_alpha_minimum: f32 = "u_alpha_minimum",
    }

    pipeline pipe_no_blend {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "u_texture",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline pipe_blend {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "u_texture",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

const TEX_COORD_A : [f32; 3] = [0.0, 0.0, 0.0];
const TEX_COORD_B : [f32; 3] = [0.0, 0.0, 1.0];
const NORMAL : [f32; 3] = [1.0, 0.0, 0.0];

const TRIANGLE: [Vertex; 3] = [
    Vertex { position: [  -0.5, -0.5, 0.0 ], tex_coord: [  0.0, 0.0, 0.0 ], color: [1.0, 1.0, 1.0, 1.0], normal: NORMAL },
    Vertex { position: [  0.5, -0.5, 0.0 ], tex_coord: [  1.0, 0.0, 0.0 ],  color: [1.0, 1.0, 1.0, 1.0], normal: NORMAL },
    Vertex { position: [  0.0,  0.5 , 0.0 ], tex_coord: [  0.5, 1.0 , 0.0 ], color: [1.0, 1.0, 1.0, 1.0], normal: NORMAL },
];

const TRIANGLE_B: [Vertex; 3] = [
    Vertex { position: [  -0.3, -0.5, -0.1 ], tex_coord: [  0.0, 0.0, 1.0 ], color: [0.0, 0.0, 1.0, 1.0], normal: NORMAL },
    Vertex { position: [  0.7, -0.5, -0.1 ], tex_coord: [  1.0, 0.0, 1.0 ],  color: [0.0, 1.0, 0.0, 1.0], normal: NORMAL },
    Vertex { position: [  0.2,  0.5 , -0.1 ], tex_coord: [  0.5, 1.0 , 1.0 ], color: [1.0, 0.0, 0.0, 1.0], normal: NORMAL },
];

pub fn ui_projection(width: f32, height: f32) -> cgmath::Matrix4<f32> {
    // left, right, bottom, top, near, far
    cgmath::ortho(0.0, width, 0.0, height, -100.0, 100.0) // having trouble with this z stuff
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn run_example() {
    println!("booting up");

    let width = 1024;
    let height = 768;
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(width, height);
    use glutin::{GlRequest, Api};
    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);

    let (window, mut device, mut factory, mut main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    println!("pre pso");
    let pso = factory.create_pipeline_simple(
        include_bytes!("../resources/shaders/fat.vert"),
        include_bytes!("../resources/shaders/fat.frag"),
        pipe_no_blend::new()
    ).unwrap();

    let pso_blend = factory.create_pipeline_simple(
        include_bytes!("../resources/shaders/fat.vert"),
        include_bytes!("../resources/shaders/fat.frag"),
        pipe_blend::new()
    ).unwrap();


    println!("pso done");

    println!("time for textures");
    let a = image::open("resources/textures/0_layer.png").unwrap().to_rgba().into_raw();
    let b = image::open("resources/textures/1_layer.png").unwrap().to_rgba().into_raw();
    let data : Vec<&[u8]> = vec![&a, &b];
    let kind = texture::Kind::D2Array(64, 64, 2, gfx::texture::AaMode::Single);
    let (texture, texture_view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &data).unwrap();

    println!("textures done");


    // we need window dimensions
//    let transform = ui_projection(width as f32, height as f32);
    let transform = cgmath::Matrix4::identity();

    let (vb_a, slice_a) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let (vb_b, slice_b) = factory.create_vertex_buffer_with_slice(&TRIANGLE_B, ());

    let sampler = factory.create_sampler_linear();

    let mut opaque_data = pipe_no_blend::Data {
        vbuf: vb_a.clone(),
        texture: (texture_view.clone(), sampler.clone()),
        locals: factory.create_constant_buffer(1),
        out_color: main_color.clone(),
        out_depth: main_depth.clone(),
    };

    let mut blend_data = pipe_blend::Data {
        vbuf: vb_b.clone(),
        texture: (texture_view.clone(), sampler.clone()),
        locals: factory.create_constant_buffer(1),
        out_color: main_color.clone(),
        out_depth: main_depth.clone(),
    };

    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } | glutin::WindowEvent::Closed => {
                        println!("closing, time to stop running");
                        running = false
                    },
                    glutin::WindowEvent::Resized(width, height) => {
                        let scale = window.hidpi_factor();

                        println!("resize to {:?} {:?} scale {:?}", width, height, scale);
                        window.resize(width, height);


                        gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                    },
                    _ => (),
                }
            }
        });



        let locals = Locals {
            u_transform: transform.into(),
            u_color: [1.0, 0.0, 0.0, 1.0],
            u_alpha_minimum: 0.1,
        };
        encoder.update_constant_buffer(&opaque_data.locals, &locals);
        encoder.update_constant_buffer(&blend_data.locals, &locals);

        // draw a frame
        encoder.clear(&main_color, CLEAR_COLOR);
        encoder.clear_depth(&main_depth, 1.0);

        opaque_data.vbuf = vb_a.clone();
        encoder.draw(&slice_a, &pso, &opaque_data);
        blend_data.vbuf = vb_b.clone();
        encoder.draw(&slice_b, &pso_blend, &blend_data);
//        blend_data.vbuf = vb_b.clone();
//        encoder.draw(&slice_b, &pso_blend, &blend_data);



        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}


