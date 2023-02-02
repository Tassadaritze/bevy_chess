use bevy::prelude::*;
use bevy::window::WindowResized;

pub mod cursor;

pub fn on_window_resize(
    mut camera_q: Query<&mut OrthographicProjection>,
    mut window_resized_ev: EventReader<WindowResized>,
) {
    for ev in window_resized_ev.iter() {
        let width_scale = (1600. / ev.width).max(1.);
        let height_scale = (900. / ev.height).max(1.);
        camera_q.single_mut().scale = width_scale.max(height_scale);
    }
}
