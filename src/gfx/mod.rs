//use ash::version::DeviceV1_2;
//use ash::version::InstanceV1_2;
//use ash::vk;
//use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
//use winit::event_loop::{EventLoop, ControlFlow};
//use train_head_engine::Config;
//
//struct WindowWrapper {
//    instance: winit::window::Window,
//    surface_loader: ash::extensions::khr::Surface,
//    surface: vk::SurfaceKHR,
//}
//
//struct GFXUtils {
//    debug_utils_loader: ash::extensions::ext::DebugUtils,
//    debug_merssager: vk::DebugUtilsMessengerEXT,
//}
//
//struct Vulkan {
//    _entry: ash::Entry,
//    instance: ash::Instance,
//    physical_device: vk::PhysicalDevice,
//    device: ash::Device,
//    queue_family: QueueFamilyIndices,
//    graphics_queue: vk::Queue,
//    present_queue: vk::Queue,
//}
//
//struct Swapchain {
//    swapchain_loader: ash::extensions::khr::Swapchain,
//    swapchain: vk::SwapchainKHR,
//    swapchain_images: Vec<vk::Image>,
//    swapchain_format: vk::Format,
//    swapchain_extent: vk::Extent2D,
//    swapchain_imageview: Vec<vk::ImageView>,
//    saccharin_framebuffer: Vec<vk::Framebuffer>,
//}
//
//pub(crate) struct GFX {
//    window: WindowWrapper,
//    utils: GFXUtils,
//    vulkan: Vulkan,
//    swapchain: Swapchain,
//
//    render_pass: vk::RenderPass,
//    pipeline_layout: vk::PipelineLayout,
//    graphics_pipeline: vk::Pipeline,
//
//    command_pool: vk::CommandPool,
//    command_buffers: Vec<vk::CommandBuffer>,
//
//    image_available_semaphores: Vec<vk::Semaphore>,
//    render_finished_semaphores: Vec<vk::Semaphore>,
//    in_flight_fences: Vec<vk::Fence>,
//    current_frame: usize,
//
//    is_framebuffer_resized: bool,
//}
//
//
//
//
//impl GFX {
//    pub fn new(config:&Config , event_loop: &winit::event_loop::EventLoop<()>) -> GFX {
//
//        let window = utility::window::init_window(event_loop, config.window_title, config.resolution.x, config.resolution.y);
//
//
//        GFX {
//            window: WindowWrapper {
//                instance: window,
//                surface_loader: surface_stuff.surface_loader,
//                surface: Default::default()
//            },
//            utils: GFXUtils {
//                debug_utils_loader: (),
//                debug_merssager: Default::default()},
//            vulkan: Vulkan {
//                _entry: (),
//                instance: (),
//                physical_device: (),
//                device: (),
//                queue_family: (),
//                graphics_queue: (),
//                present_queue: ()
//            },
//            swapchain: Swapchain {
//                swapchain_loader: (),
//                swapchain: Default::default(),
//                swapchain_images: vec![],
//                swapchain_format: Default::default(),
//                swapchain_extent: Default::default(),
//                swapchain_imageview: vec![],
//                saccharin_framebuffer: vec![]
//            },
//            render_pass: Default::default(),
//            pipeline_layout: Default::default(),
//            graphics_pipeline: Default::default(),
//            command_pool: Default::default(),
//            command_buffers: vec![],
//            image_available_semaphores: vec![],
//            render_finished_semaphores: vec![],
//            in_flight_fences: vec![],
//            current_frame: 0,
//            is_framebuffer_resized: false
//        }
//    }
//}