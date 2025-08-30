use cgmath::prelude::*;
use three_d::{degrees, Camera, Event, MouseButton, Vec3};

///
/// A control that makes the camera orbit around a target.
///
#[derive(Clone, Copy, Debug)]
pub struct OrbitControl {
    /// The target point to orbit around.
    pub target: Vec3,
    /// The minimum distance to the target point.
    pub min_distance: f32,
    /// The maximum distance to the target point.
    pub max_distance: f32,
}

impl OrbitControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            target,
            min_distance,
            max_distance,
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion { delta, button, handled, .. } => {
                    if let Some(button) = button {
                        if !*handled {
                            if self.apply_mouse_motion(camera, *delta, *button) {
                                *handled = true;
                                change = true;
                            }
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        let dist = self.target.distance(camera.position());
                        self.apply_zoom(camera, delta.1, 0.001 * dist);
                        *handled = true;
                        change = true;
                    }
                }
                _ => {}
            }
        }
        change
    }

    fn apply_mouse_motion(
        &self, camera: &mut Camera, delta: (f32, f32), button: MouseButton
    ) -> bool
    {
        let (x, y) = delta;

        match button {
            MouseButton::Left => {
                let speed = 0.1;
                camera.rotate_around(self.target, speed * x, speed * y);
                true
            }
            MouseButton::Middle => {
                let speed = 0.01;
                let shift = -camera.right_direction() * x + camera.up_orthogonal() * y;
                camera.translate(speed * shift);
                true
            }
            MouseButton::Right => {
                let speed = 0.1;
                camera.roll(degrees(speed * x));
                true
            }
        }
    }

    fn apply_zoom(&self, camera: &mut Camera, delta: f32, speed: f32) {
        camera.zoom_towards(
            self.target, speed * delta, self.min_distance, self.max_distance,
        );
    }
}
