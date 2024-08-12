use std::iter;

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
use sqlx::{
    migrate::MigrateDatabase, Pool, Sqlite, SqlitePool
};

use crate::{database::{create_tables, fetch_all_nodes_and_tags, fetch_all_renderable_ways}, fetcher::read_openstreet_map_file, osm_entities::{Node, RenderableWay}, texture, utils::lat_lon_to_screen, DB_URL};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    top_left_corner: (f64, f64),
    bottom_right_corner: (f64, f64),
    renderable_ways : Vec<RenderableWay>,
    pool: Pool<Sqlite>,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        // We start by making sure there is a database to connect to
        // Create a database instance with the full connection string.
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await;
            println!("Database created successfully");
        } else {
            println!("Database already exists");
        }
        let pool = SqlitePool::connect(DB_URL).await.unwrap();
        create_tables(&pool).await;
        println!("Tables created successfully");

        // // Read and process the chosen map file
        // read_openstreet_map_file(&pool).await;

        let top_left_corner: (f64, f64) = (55.0407000, 11.3377000);
        let bottom_right_corner: (f64, f64) = (55.0210000, 11.3794000);

        // Get the renderable ways from the database
        let renderable_ways = match fetch_all_renderable_ways(&pool).await {
            Ok(renderable_ways) => renderable_ways,
            Err(error) => panic!("There was a problem fetching the renderable ways: {:?}", error),
        };

        println!("There are {} renderable_ways", renderable_ways.len());

        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let building_texture_bytes = include_bytes!("../utils/textures/building.png");
        let highway_texture_bytes = include_bytes!("../utils/textures/highway.png");
        let coastline_texture_bytes = include_bytes!("../utils/textures/coastline.png");

        let building_texture = texture::Texture::from_bytes(&device, &queue, building_texture_bytes, "building.png").unwrap();
        let highway_texture = texture::Texture::from_bytes(&device, &queue, highway_texture_bytes, "highway.png").unwrap();
        let coastline_texture = texture::Texture::from_bytes(&device, &queue, coastline_texture_bytes, "coastline.png").unwrap();

        let diffuse_bytes = include_bytes!("../utils/textures/node.png");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let (vertices, indices) = generate_vertices_and_indices_from_renderable_ways(&renderable_ways, top_left_corner, bottom_right_corner);

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = indices.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            renderable_ways,
            pool,
            top_left_corner,
            bottom_right_corner,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // TODO
    }

    fn update_buffers(&mut self) {
        // Generate vertices and indices from renderable_ways
        let (vertices, indices) = generate_vertices_and_indices_from_renderable_ways(&self.renderable_ways, self.top_left_corner, self.bottom_right_corner);

        // Update the vertex buffer with the node vertices
        self.vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Node Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // Update the index buffer with the node indices
        self.index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Node Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        self.num_indices = indices.len() as u32;
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn generate_vertices_and_indices_from_renderable_ways(renderable_ways: &Vec<RenderableWay>, top_left: (f64, f64), bottom_right: (f64, f64)) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for way in renderable_ways {
        // Determine how to visualize this way based on its tags
        let is_coastline = way.tags.iter().any(|tag| tag.key == "natural" && tag.value == "coastline");
        let is_highway = way.tags.iter().any(|tag| tag.key == "highway" && tag.value == "track");
        let is_building = way.tags.iter().any(|tag| tag.key == "building");

        if is_coastline {
            // Handle coastline rendering (e.g., as lines)
            generate_line_vertices_and_indices(way, top_left, bottom_right, 0.002, &mut vertices, &mut indices);
        } else if is_highway {
            // Handle highway rendering (e.g., as thick lines or strips)
            generate_line_vertices_and_indices(way, top_left, bottom_right, 0.005, &mut vertices, &mut indices);
        } else if is_building {
            // Handle building rendering (e.g., as polygons)
            generate_polygon_vertices_and_indices(way, top_left, bottom_right, &mut vertices, &mut indices);
        } else {
            // Handle other types of ways or default rendering (e.g., as lines)
            generate_line_vertices_and_indices(way, top_left, bottom_right, 0.002, &mut vertices, &mut indices);
        }
    }
    // println!("{:#?}", vertices);
    (vertices, indices)
}

fn generate_line_vertices_and_indices(
    way: &RenderableWay,
    top_left: (f64, f64),
    bottom_right: (f64, f64),
    thickness: f32, // Parameter to control the thickness
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
) {
    let base_index = vertices.len() as u16;

    // Loop through the nodes in the way
    for (i, node) in way.nodes.iter().enumerate() {
        let (x, y) = lat_lon_to_screen(node.lat, node.lon, top_left, bottom_right);

        // If this isn't the first node, calculate direction to the previous node
        if i > 0 {
            let (prev_x, prev_y) = lat_lon_to_screen(
                way.nodes[i - 1].lat,
                way.nodes[i - 1].lon,
                top_left,
                bottom_right,
            );

            // Calculate the direction vector from the previous point to the current point
            let direction = (
                x - prev_x,
                y - prev_y,
            );

            // Normalize the direction vector
            let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
            let direction = (
                direction.0 / length,
                direction.1 / length,
            );

            // Calculate the perpendicular vector to the line direction
            let perpendicular = (
                -direction.1 * thickness / 2.0,
                direction.0 * thickness / 2.0,
            );

            // Define the vertices for the thick line
            vertices.push(Vertex {
                position: [prev_x + perpendicular.0, prev_y + perpendicular.1, 0.0],
                tex_coords: [0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [prev_x - perpendicular.0, prev_y - perpendicular.1, 0.0],
                tex_coords: [1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x + perpendicular.0, y + perpendicular.1, 0.0],
                tex_coords: [0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x - perpendicular.0, y - perpendicular.1, 0.0],
                tex_coords: [1.0, 1.0],
            });

            // Add the indices to create two triangles forming a quad
            indices.extend_from_slice(&[
                base_index + (i as u16 - 1) * 4,
                base_index + (i as u16 - 1) * 4 + 1,
                base_index + i as u16 * 4,

                base_index + i as u16 * 4,
                base_index + (i as u16 - 1) * 4 + 1,
                base_index + i as u16 * 4 + 1,
            ]);
        }
    }

    // Connect the last node to the first node to close the loop
    if way.nodes.len() > 1 {
        let first_node = &way.nodes[0];
        let last_node = &way.nodes[way.nodes.len() - 1];

        let (x1, y1) = lat_lon_to_screen(first_node.lat, first_node.lon, top_left, bottom_right);
        let (x2, y2) = lat_lon_to_screen(last_node.lat, last_node.lon, top_left, bottom_right);

        // Calculate the direction vector from the last point to the first point
        let direction = (
            x1 - x2,
            y1 - y2,
        );

        // Normalize the direction vector
        let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
        let direction = (
            direction.0 / length,
            direction.1 / length,
        );

        // Calculate the perpendicular vector to the line direction
        let perpendicular = (
            -direction.1 * thickness / 2.0,
            direction.0 * thickness / 2.0,
        );

        // Define the vertices for the thick line from last node to first node
        vertices.push(Vertex {
            position: [x2 + perpendicular.0, y2 + perpendicular.1, 0.0],
            tex_coords: [0.0, 0.0],
        });
        vertices.push(Vertex {
            position: [x2 - perpendicular.0, y2 - perpendicular.1, 0.0],
            tex_coords: [1.0, 0.0],
        });
        vertices.push(Vertex {
            position: [x1 + perpendicular.0, y1 + perpendicular.1, 0.0],
            tex_coords: [0.0, 1.0],
        });
        vertices.push(Vertex {
            position: [x1 - perpendicular.0, y1 - perpendicular.1, 0.0],
            tex_coords: [1.0, 1.0],
        });

        // Add the indices to create two triangles forming a quad to close the loop
        indices.extend_from_slice(&[
            base_index + (way.nodes.len() as u16 - 1) * 4,
            base_index + (way.nodes.len() as u16 - 1) * 4 + 1,
            base_index,

            base_index,
            base_index + (way.nodes.len() as u16 - 1) * 4 + 1,
            base_index + 1,
        ]);
    }
}

fn generate_polygon_vertices_and_indices(way: &RenderableWay, top_left: (f64, f64), bottom_right: (f64, f64), vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>) {
    let base_index = vertices.len() as u16;

    for node in &way.nodes {
        let (x, y) = lat_lon_to_screen(node.lat, node.lon, top_left, bottom_right);
        vertices.push(Vertex {
            position: [x, y, 0.0],
            tex_coords: [0.0, 0.0], // Placeholder texture coordinates
        });
    }

    // Triangulation: For a simple polygon, assume that nodes are ordered and define a fan from the first vertex
    for i in 1..way.nodes.len() as u16 - 1 {
        indices.extend_from_slice(&[
            base_index, base_index + i, base_index + i + 1,
        ]);
    }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        // UPDATED!
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                log::info!("physical_size: {physical_size:?}");
                                surface_configured = true;
                                state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                state.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                state.update();
                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
