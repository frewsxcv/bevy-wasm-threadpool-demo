use bevy::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    let (sender, receiver) = async_channel::unbounded::<String>();
    App::new()
        .insert_resource(sender)
        .insert_resource(receiver)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(4))
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::window::WindowPlugin::default())
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::winit::WinitPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::render::RenderPlugin::default())
        .add_plugin(bevy::core_pipeline::CorePipelinePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::sprite::SpritePlugin::default())
        .add_startup_system(spawn_tasks)
        .add_system(handle_tasks)
        .run();
}

fn spawn_tasks(
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    sender: ResMut<async_channel::Sender<String>>,
) {
    web_sys::console::log_1(&"Spawning tasks".into());
    for _ in 0..20 {
        let sender = sender.clone();
        // Spawn new task on the AsyncComputeTaskPool
        thread_pool.spawn(async move {
            // let duration = std::time::Duration::from_secs_f32(5.);
            web_sys::console::log_1(&"Spawned a task".into());
            sender.send(String::from("Some string")).await.unwrap();
        });
    }
}

fn handle_tasks(receiver: Res<async_channel::Receiver<String>>) {
    while let Ok(string) = receiver.try_recv() {
        web_sys::console::log_1(&format!("Handled task! {}", string).into());
    }
}
