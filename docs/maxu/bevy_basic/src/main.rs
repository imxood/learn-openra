use bevy::{
    app::ScheduleRunnerPlugin,
    log::{Level, LogPlugin, LogSettings},
    prelude::*,
};

///
/// ECS: Entity, Component, System
/// 实体(Entity) 是指向一群 组件(Component) 的唯一 事物(things), 然后使用 系统(System) 处理其过程.
/// 资源(Resources) 来表示全局唯一数据: 时钟, 资源集合 (声音, 纹理, 网格), 渲染器 等

// Stage 表示

// systems

// app 对象

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_event::<StartEvent>()
        .init_resource::<Time>()
        .insert_resource(LogSettings {
            filter: "wgpu=error".to_string(),
            level: Level::TRACE,
        })
        .insert_resource(MyTimer {
            timer: Timer::from_seconds(5.0, true),
        })
        .add_plugin(LogPlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_system(timer_system.system())
        .add_system(event_system.system())
        .add_system_to_stage(CoreStage::First, time_system.system())
        .run();
    println!("Hello, world!");
}

#[derive(Debug)]
struct StartEvent {}

struct MyTimer {
    timer: Timer,
}

fn setup(commands: &mut Commands) {
    commands
        // cameras
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default());
}

fn event_system(mut start_events: EventReader<StartEvent>) {
    for event in start_events.iter() {
        println!("{:?} received", event);
    }
}

fn timer_system(
    time: Res<Time>,
    mut my_timer: ResMut<MyTimer>,
    mut start_event: EventWriter<StartEvent>,
) {
    if my_timer.timer.tick(time.delta()).just_finished() {
        start_event.send(StartEvent {})
    }
}

fn time_system(mut time: ResMut<Time>) {
    time.update();
}
