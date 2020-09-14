use winit::event_loop::EventLoop;
use engine::config::Config;
use train_head_engine::Resolution;
use json::JsonValue;

mod gfx;
mod engine;

fn main() {
    let event_loop = EventLoop::new();
    const CONFIG : JsonValue = Config::new("Hello");

    let res = Resolution::new(2560, 1080);

//    let vulkan_app = GFX::new(&event_loop);
//    vulkan_app.main_loop(event_loop);
}