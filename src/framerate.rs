use bevy::prelude::*;

pub struct FramerateMonitorPlugin;

impl Plugin for FramerateMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Framerate(0.0))
            .insert_resource(FramerateIsStable(false))
            .add_systems(Update, update_framerate);
    }
}

#[derive(Resource)]
pub struct Framerate(pub f32);

#[derive(Resource)]
pub struct FramerateIsStable(pub bool);

fn update_framerate(
    mut framerate: ResMut<Framerate>,
    time: Res<Time>,
    mut stable: ResMut<FramerateIsStable>,
) {
    let d = time.delta_seconds().max(0.0001);

    if d > 0.1 {
        warn!("slow frame {}", d);
    }

    let n = d * 0.5;
    framerate.0 = framerate.0 * (1.0 - n) + (1.0 / d) * n;

    let min_stable_fps = 20.0;

    if framerate.0 > min_stable_fps && stable.0 == false {
        stable.0 = true;
        info!("framerate is now stable");
    } else if framerate.0 < min_stable_fps && stable.0 == true {
        stable.0 = false;
        warn!("framerate is now unstable");
    }
}
