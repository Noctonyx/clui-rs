use log::LevelFilter;
use std::path::Path;
use std::time::Instant;

use rafx_api::{
    RafxApi, RafxBuffer, RafxBufferDef, RafxColorClearValue, RafxColorRenderTargetBinding,
    RafxCommandBuffer, RafxCommandBufferDef, RafxCommandPool, RafxCommandPoolDef,
    RafxDescriptorElements, RafxDescriptorKey, RafxDescriptorSetArray, RafxDescriptorSetArrayDef,
    RafxDescriptorUpdate, RafxDeviceContext, RafxError, RafxFormat, RafxGlUniformMember,
    RafxGraphicsPipelineDef, RafxLoadOp, RafxPipeline, RafxPrimitiveTopology, RafxQueue,
    RafxQueueType, RafxResourceState, RafxResourceType, RafxResult, RafxRootSignatureDef,
    RafxSampleCount, RafxShader, RafxShaderPackage, RafxShaderPackageVulkan, RafxShaderResource,
    RafxShaderStageDef, RafxShaderStageFlags, RafxShaderStageReflection, RafxStoreOp,
    RafxSwapchainColorSpace, RafxSwapchainDef, RafxSwapchainHelper, RafxTextureBarrier,
    RafxVertexAttributeRate, RafxVertexBufferBinding, RafxVertexLayout, RafxVertexLayoutAttribute,
    RafxVertexLayoutBuffer,
};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

#[derive(Default, Clone, Copy)]
struct PositionColorVertex {
    position: [f32; 2],
    color: [f32; 3],
}

struct DemoGraphicsContext {
    device_context: RafxDeviceContext,
    swapchain_helper: RafxSwapchainHelper,
    graphics_queue: RafxQueue,
    //shader: RafxShader,
    //root_signature: RafxRootSignature,
    //vertex_layout: RafxVertexLayout,
    pipeline: RafxPipeline,
    command_pools: Vec<RafxCommandPool>,
    command_buffers: Vec<RafxCommandBuffer>,
    vertex_buffers: Vec<Option<RafxBuffer>>,
    uniform_buffers: Vec<Option<RafxBuffer>>,
    descriptor_set_array: RafxDescriptorSetArray,
}

struct DemoApp {
    api: RafxApi,
    start_time: Instant,
    graphics: Option<DemoGraphicsContext>,
}

impl DemoApp {
    fn init(window: &Window) -> Result<Self, RafxError> {
        let api = unsafe { RafxApi::new(window, &Default::default())? };

        let graphics = create_graphics_context(&api, window)?;

        Ok(DemoApp {
            api,
            start_time: std::time::Instant::now(),
            graphics: Some(graphics),
        })
    }

    fn update(&mut self, window: &Window) -> Result<winit::event_loop::ControlFlow, RafxError> {
        let ctx = self.graphics.as_mut().unwrap();

        let elapsed_seconds = self.start_time.elapsed().as_secs_f32();

        //
        // Acquire swapchain image
        //
        let physical_size = window.inner_size();
        let presentable_frame = ctx.swapchain_helper.acquire_next_image(
            physical_size.width,
            physical_size.height,
            None,
        )?;
        let swapchain_texture = presentable_frame.swapchain_texture();

        let cmd_pool = &mut ctx.command_pools[presentable_frame.rotating_frame_index()];
        let cmd_buffer = &ctx.command_buffers[presentable_frame.rotating_frame_index()];

        #[rustfmt::skip]
            let vertex_data = [
            0.0f32, 0.5, 1.0, 0.0, 0.0,
            0.5 - (elapsed_seconds.cos() / 2. + 0.5), -0.5, 0.0, 1.0, 0.0,
            -0.5 + (elapsed_seconds.cos() / 2. + 0.5), -0.5, 0.0, 0.0, 1.0,
        ];

        let color = (elapsed_seconds.cos() + 1.0) / 2.0;
        let uniform_data = [color, 0.0, 1.0 - color, 1.0];

        let vertex_buffer = ctx
            .device_context
            .create_buffer(&RafxBufferDef::for_staging_vertex_buffer_data(&vertex_data))?;
        vertex_buffer.copy_to_host_visible_buffer(&vertex_data)?;

        let uniform_buffer =
            ctx.device_context
                .create_buffer(&RafxBufferDef::for_staging_uniform_buffer_data(
                    &uniform_data,
                ))?;
        uniform_buffer.copy_to_host_visible_buffer(&uniform_data)?;

        ctx.descriptor_set_array
            .update_descriptor_set(&[RafxDescriptorUpdate {
                array_index: presentable_frame.rotating_frame_index() as u32,
                descriptor_key: RafxDescriptorKey::Name("color"),
                elements: RafxDescriptorElements {
                    buffers: Some(&[&uniform_buffer]),
                    ..Default::default()
                },
                ..Default::default()
            }])?;

        cmd_pool.reset_command_pool()?;
        cmd_buffer.begin()?;

        // Put it into a layout where we can draw on it
        cmd_buffer.cmd_resource_barrier(
            &[],
            &[RafxTextureBarrier::state_transition(
                &swapchain_texture,
                RafxResourceState::PRESENT,
                RafxResourceState::RENDER_TARGET,
            )],
        )?;

        cmd_buffer.cmd_begin_render_pass(
            &[RafxColorRenderTargetBinding {
                texture: &swapchain_texture,
                load_op: RafxLoadOp::Clear,
                store_op: RafxStoreOp::Store,
                array_slice: None,
                mip_slice: None,
                clear_value: RafxColorClearValue([0.2, 0.2, 0.2, 1.0]),
                resolve_target: None,
                resolve_store_op: RafxStoreOp::DontCare,
                resolve_mip_slice: None,
                resolve_array_slice: None,
            }],
            None,
        )?;

        cmd_buffer.cmd_bind_pipeline(&ctx.pipeline)?;

        cmd_buffer.cmd_bind_vertex_buffers(
            0,
            &[RafxVertexBufferBinding {
                buffer: &vertex_buffer,
                byte_offset: 0,
            }],
        )?;
        cmd_buffer.cmd_bind_descriptor_set(
            &ctx.descriptor_set_array,
            presentable_frame.rotating_frame_index() as u32,
        )?;

        cmd_buffer.cmd_draw(3, 0)?;

        cmd_buffer.cmd_end_render_pass()?;

        cmd_buffer.cmd_resource_barrier(
            &[],
            &[RafxTextureBarrier::state_transition(
                &swapchain_texture,
                RafxResourceState::RENDER_TARGET,
                RafxResourceState::PRESENT,
            )],
        )?;
        cmd_buffer.end()?;

        ctx.vertex_buffers[presentable_frame.rotating_frame_index()] = Some(vertex_buffer);
        ctx.uniform_buffers[presentable_frame.rotating_frame_index()] = Some(uniform_buffer);

        //
        // Present the image
        //
        presentable_frame.present(&ctx.graphics_queue, &[&cmd_buffer])?;

        Ok(ControlFlow::Poll)
    }

    fn process_input(
        &mut self,
        event: &winit::event::Event<()>,
        _window: &winit::window::Window,
    ) -> bool {
        let ignore_event: bool = false;

        if !ignore_event {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => return false,
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(virtual_keycode),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    if *virtual_keycode == VirtualKeyCode::Escape {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn shutdown(&mut self) {
        self.graphics = None;
    }
}

impl Drop for DemoApp {
    fn drop(&mut self) {
        self.shutdown()
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .default_format()
        .filter_level(LevelFilter::Info)
        .filter_module("rafx_api::backends::vulkan", LevelFilter::Info)
        .init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ClUI Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1600, 900))
        .build(&event_loop)
        .unwrap();

    let mut app = DemoApp::init(&window).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                *control_flow = app.update(&window).unwrap();
            }
            event @ _ => {
                if !app.process_input(&event, &window) {
                    *control_flow = ControlFlow::Exit;
                }
            }
        }
    });
}

fn create_graphics_context(
    api: &RafxApi,
    window: &Window,
) -> Result<DemoGraphicsContext, RafxError> {
    let device_context = api.device_context();

    let physical_size = window.inner_size();

    let swapchain = device_context.create_swapchain(
        window,
        &RafxSwapchainDef {
            width: physical_size.width,
            height: physical_size.height,
            enable_vsync: true,
            color_space_priority: vec![RafxSwapchainColorSpace::Srgb],
        },
    )?;

    let swapchain_helper = RafxSwapchainHelper::new(&device_context, swapchain, None)?;
    let graphics_queue = device_context.create_queue(RafxQueueType::Graphics)?;

    let mut command_pools = Vec::with_capacity(swapchain_helper.image_count());
    let mut command_buffers = Vec::with_capacity(swapchain_helper.image_count());
    let mut vertex_buffers: Vec<Option<RafxBuffer>> =
        Vec::with_capacity(swapchain_helper.image_count());
    let mut uniform_buffers: Vec<Option<RafxBuffer>> =
        Vec::with_capacity(swapchain_helper.image_count());

    for _ in 0..swapchain_helper.image_count() {
        let mut command_pool =
            graphics_queue.create_command_pool(&RafxCommandPoolDef { transient: true })?;

        let command_buffer = command_pool.create_command_buffer(&RafxCommandBufferDef {
            is_secondary: false,
        })?;

        command_pools.push(command_pool);
        command_buffers.push(command_buffer);
        vertex_buffers.push(None);
        uniform_buffers.push(None);
    }

    let shader = create_shader(&device_context)?;

    //
    // Create the root signature object - it represents the pipeline layout and can be shared among
    // shaders. But one per shader is fine.
    //
    let root_signature = device_context.create_root_signature(&RafxRootSignatureDef {
        shaders: &[shader.clone()],
        immutable_samplers: &[],
    })?;

    //
    // Descriptors are allocated in blocks and never freed. Normally you will want to build a
    // pooling system around this. (Higher-level rafx crates provide this.) But they're small
    // and cheap. We need one per swapchain image.
    //
    let descriptor_set_array =
        device_context.create_descriptor_set_array(&RafxDescriptorSetArrayDef {
            set_index: 0,
            root_signature: &root_signature,
            array_length: swapchain_helper.image_count(), // One per swapchain image.
        })?;

    //
    // Now set up the pipeline. LOTS of things can be configured here, but aside from the vertex
    // layout most of it can be left as default.
    //
    let vertex_layout = RafxVertexLayout {
        attributes: vec![
            RafxVertexLayoutAttribute {
                format: RafxFormat::R32G32_SFLOAT,
                buffer_index: 0,
                location: 0,
                byte_offset: 0,
                gl_attribute_name: Some("pos".to_string()),
            },
            RafxVertexLayoutAttribute {
                format: RafxFormat::R32G32B32_SFLOAT,
                buffer_index: 0,
                location: 1,
                byte_offset: 8,
                gl_attribute_name: Some("in_color".to_string()),
            },
        ],
        buffers: vec![RafxVertexLayoutBuffer {
            stride: 20,
            rate: RafxVertexAttributeRate::Vertex,
        }],
    };

    let pipeline = device_context.create_graphics_pipeline(&RafxGraphicsPipelineDef {
        shader: &shader,
        root_signature: &root_signature,
        vertex_layout: &vertex_layout,
        blend_state: &Default::default(),
        depth_state: &Default::default(),
        rasterizer_state: &Default::default(),
        color_formats: &[swapchain_helper.format()],
        sample_count: RafxSampleCount::SampleCount1,
        depth_stencil_format: None,
        primitive_topology: RafxPrimitiveTopology::TriangleList,
    })?;

    Ok(DemoGraphicsContext {
        device_context,
        swapchain_helper,
        graphics_queue,
        //shader,
        //root_signature,
        //vertex_layout,
        pipeline,
        command_pools,
        command_buffers,
        vertex_buffers,
        uniform_buffers,
        descriptor_set_array,
    })
}

fn create_shader(device_context: &RafxDeviceContext) -> Result<RafxShader, RafxError> {
    let processed_shaders_base_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("processed_shaders");

    let vert_shader_package = load_shader_packages(
        &processed_shaders_base_path,
        "shader.vert.metal",
        "shader.vert.spv",
        "shader.vert.gles2",
        "shader.vert.gles3",
    )?;

    let frag_shader_package = load_shader_packages(
        &processed_shaders_base_path,
        "shader.frag.metal",
        "shader.frag.spv",
        "shader.frag.gles2",
        "shader.frag.gles3",
    )?;

    let vert_shader_module =
        device_context.create_shader_module(vert_shader_package.module_def())?;
    let frag_shader_module =
        device_context.create_shader_module(frag_shader_package.module_def())?;

    let color_shader_resource = RafxShaderResource {
        name: Some("color".to_string()),
        set_index: 0,
        binding: 0,
        resource_type: RafxResourceType::UNIFORM_BUFFER,
        gles_name: Some("UniformData".to_string()),
        gles2_uniform_members: vec![RafxGlUniformMember::new("UniformData.uniform_color", 0)],
        ..Default::default()
    };

    let vert_shader_stage_def = RafxShaderStageDef {
        shader_module: vert_shader_module,
        reflection: RafxShaderStageReflection {
            entry_point_name: "main".to_string(),
            shader_stage: RafxShaderStageFlags::VERTEX,
            compute_threads_per_group: None,
            resources: vec![color_shader_resource.clone()],
        },
    };

    let frag_shader_stage_def = RafxShaderStageDef {
        shader_module: frag_shader_module,
        reflection: RafxShaderStageReflection {
            entry_point_name: "main".to_string(),
            shader_stage: RafxShaderStageFlags::FRAGMENT,
            compute_threads_per_group: None,
            resources: vec![color_shader_resource],
        },
    };

    let shader =
        device_context.create_shader(vec![vert_shader_stage_def, frag_shader_stage_def])?;

    Ok(shader)
}

fn load_shader_packages(
    _base_path: &Path,
    _metal_src_file: &str,
    _vk_spv_file: &str,
    _gles2_src_file: &str,
    _gles3_src_file: &str,
) -> RafxResult<RafxShaderPackage> {
    let mut _package = RafxShaderPackage::default();

    #[cfg(feature = "rafx-metal")]
    {
        let metal_path = _base_path.join(_metal_src_file);
        let metal_src = std::fs::read_to_string(metal_path)?;
        _package.metal = Some(RafxShaderPackageMetal::Src(metal_src));
    }

    #[cfg(feature = "rafx-vulkan")]
    {
        let vk_path = _base_path.join(_vk_spv_file);
        let vk_bytes = std::fs::read(vk_path)?;
        _package.vk = Some(RafxShaderPackageVulkan::SpvBytes(vk_bytes));
    }

    #[cfg(feature = "rafx-gles2")]
    {
        let gles2_path = _base_path.join(_gles2_src_file);
        let gles2_src = std::fs::read_to_string(gles2_path)?;
        _package.gles2 = Some(RafxShaderPackageGles2::Src(gles2_src));
    }

    #[cfg(feature = "rafx-gles3")]
    {
        let gles3_path = _base_path.join(_gles3_src_file);
        let gles3_src = std::fs::read_to_string(gles3_path)?;
        _package.gles3 = Some(RafxShaderPackageGles3::Src(gles3_src));
    }

    Ok(_package)
}
