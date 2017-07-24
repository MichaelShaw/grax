extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_device_gl;

use cgmath::SquareMatrix;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::texture;
use glutin::GlContext;
use gfx::format::{Depth, Rgba8};
use image::{GenericImage, RgbaImage};
use gfx::Factory;
use gfx::texture::ImageInfoCommon;
use gfx::format::R8_G8_B8_A8;

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


pub struct App<R, C, F, D> where R : gfx::Resources,
                                 C : gfx::CommandBuffer<R>,
                                 F : gfx::Factory<R>,
                                 D : Device {
    pub window: glutin::GlWindow,
    pub events_loop: glutin::EventsLoop,
    pub device: D,
    pub factory: F,
    pub screen_colour_target: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub screen_depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub encoder: gfx::Encoder<R, C>,
    pub texture: Option<gfx::handle::Texture<R, Rgba8>>,
    pub texture_view: Option<gfx::handle::ShaderResourceView<R, [f32; 4]>>,
    pub sampler: gfx::handle::Sampler<R>,
    // programs
    // textures
}

pub struct NoBlendPSO<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_no_blend::Meta>,
    pub data : pipe_no_blend::Data<R>,
}

pub struct BlendPSO<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_blend::Meta>,
    pub data : pipe_blend::Data<R>,
}

pub fn construct_gl_app() -> App<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, gfx_device_gl::Factory, gfx_device_gl::Device> {
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

    //    context = 4;

    let (window, mut device, mut factory, mut main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let sampler = factory.create_sampler_linear();

    App {
        window,
        events_loop,
        device,
        factory,
        screen_colour_target: main_color,
        screen_depth_target: main_depth,
        encoder: encoder,
        texture: None,
        texture_view: None,
        sampler,
    }
}


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
    println!("scale at boot -> {:?}", window.hidpi_factor());

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    println!("pre pso");

    let sampler = factory.create_sampler_linear();




    println!("pso done");

    println!("time for textures");
    let a = image::open("resources/textures/0_layer.png").unwrap().to_rgba().into_raw();
    let b = image::open("resources/textures/1_layer.png").unwrap().to_rgba().into_raw();

    let data : Vec<&[u8]> = vec![&a, &b];
    let kind = texture::Kind::D2Array(64, 64, 2, gfx::texture::AaMode::Single);
    let (texture, texture_view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &data).unwrap();
    // texture: gfx::handle::Texture<gfx_device_gl::Resources, gfx::format::R8_G8_B8_A8>
    // texture_view: gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>


    // fn create_texture<S>(&mut self, kind: texture::Kind, levels: target::Level,
//    bind: Bind, usage: Usage, channel_hint: Option<format::ChannelType>)
    let bind = gfx::SHADER_RESOURCE;
    let cty = gfx::format::ChannelType::Unorm;
    let tex_mut = factory.create_texture(kind, 1, bind, gfx::memory::Usage::Dynamic, Some(cty)).unwrap();
    let tex_mut_view = factory.view_texture_as_shader_resource::<Rgba8>(&tex_mut, (0, 0), gfx::format::Swizzle::new()).unwrap();

    let mut texture_uploaded = false;

    println!("textures done");


    // we need window dimensions
//    let transform = ui_projection(width as f32, height as f32);
    let transform = cgmath::Matrix4::identity();

    let (vb_a, slice_a) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let (vb_b, slice_b) = factory.create_vertex_buffer_with_slice(&TRIANGLE_B, ());

//    vb_a = 4; // `gfx::handle::Buffer<gfx_device_gl::Resources, Vertex>`
//    slice_a = 8; // `gfx::Slice<gfx_device_gl::Resources>`

    let pso = factory.create_pipeline_simple(
        include_bytes!("../resources/shaders/fat.vert"),
        include_bytes!("../resources/shaders/fat.frag"),
        pipe_no_blend::new()
    ).unwrap();
    let opaque_data = pipe_no_blend::Data {
        vbuf: vb_a.clone(),
        texture: (tex_mut_view.clone(), sampler.clone()),
        locals: factory.create_constant_buffer(1),
        out_color: main_color.clone(),
        out_depth: main_depth.clone(),
    };
    let mut opaque_pso = NoBlendPSO {
        pipeline: pso,
        data : opaque_data,
    };

    let pso_blend = factory.create_pipeline_simple(
        include_bytes!("../resources/shaders/fat.vert"),
        include_bytes!("../resources/shaders/fat.frag"),
        pipe_blend::new()
    ).unwrap();
    let mut blend_data = pipe_blend::Data {
        vbuf: vb_b.clone(),
        texture: (tex_mut_view.clone(), sampler.clone()),
        locals: factory.create_constant_buffer(1),
        out_color: main_color.clone(),
        out_depth: main_depth.clone(),
    };

    let mut blend_pso = BlendPSO {
        pipeline: pso_blend,
        data : blend_data
    };

    let mut n = 0;

    let mut running = true;
    while running {
        n += 1;

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
        encoder.update_constant_buffer(&opaque_pso.data.locals, &locals);
        encoder.update_constant_buffer(&blend_pso.data.locals, &locals);

        if !texture_uploaded {
            println!("uploading textures");
            let mut image_info = ImageInfoCommon {
                xoffset: 0,
                yoffset: 0,
                zoffset: 0,
                width: 64,
                height: 64,
                depth: 1,
                format: (),
                mipmap: 0,
            };

            let mut data : Vec<[u8; 4]> = a.chunks(4).map(|sl| [sl[0], sl[1], sl[2], sl[3]]).collect();

            encoder.update_texture::<R8_G8_B8_A8, Rgba8>(
                &tex_mut,
                None,
                image_info,
                &data,
            ).unwrap();

            texture_uploaded = true;
        }

        let mut image_info = ImageInfoCommon {
            xoffset: 0,
            yoffset: 0,
            zoffset: 1,
            width: 64,
            height: 64,
            depth: 1,
            format: (),
            mipmap: 0,
        };

        let mut data : Vec<[u8; 4]> = Vec::new();
        for i in 0..(64 * 64) {
            let mn = (n % 255) as u8;
            data.push([mn, mn, mn, 255]);
        }
        encoder.update_texture::<R8_G8_B8_A8, Rgba8>(
            &tex_mut,
            None,
            image_info,
            &data,
        ).unwrap();

        // draw a frame
        encoder.clear(&main_color, CLEAR_COLOR);
        encoder.clear_depth(&main_depth, 1.0);

        opaque_pso.data.vbuf = vb_a.clone();
        encoder.draw(&slice_a, &opaque_pso.pipeline, &opaque_pso.data);
        blend_data.vbuf = vb_b.clone();
        encoder.draw(&slice_b, &blend_pso.pipeline, &blend_pso.data);
//        blend_data.vbuf = vb_b.clone();
//        encoder.draw(&slice_b, &pso_blend, &blend_data);



        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}


