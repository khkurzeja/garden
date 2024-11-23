#![allow(dead_code)]

use winit::event::MouseButton;
use winit::keyboard::KeyCode;

pub const NUM_MOUSE_BUTTONS: usize = 16;
pub const NUM_KEYS: usize = 256;
pub const NUM_FRAMES: usize = 3;

pub struct Input {
    pub mouse_down: [[bool; NUM_MOUSE_BUTTONS]; NUM_FRAMES],
    pub mouse_position: [[f64; 2]; NUM_FRAMES],
    pub mouse_wheel_delta: [f64; 2],
    pub key_down: [[bool; NUM_KEYS]; NUM_FRAMES],
}
impl Input {
    pub fn new() -> Self {
        Self {
            mouse_down: [[false; NUM_MOUSE_BUTTONS]; NUM_FRAMES],
            mouse_position: [[0.0, 0.0]; NUM_FRAMES],
            mouse_wheel_delta: [0.0, 0.0],
            key_down: [[false; NUM_KEYS]; NUM_FRAMES],
        }
    }

    pub fn end_frame(&mut self) {
        self.mouse_down.rotate_right(1);
        self.mouse_position.rotate_right(1);
        self.key_down.rotate_right(1);

        // Generally, assume state has not changed since previous frame. However, mouse wheel always resets, because it represents change.
        self.mouse_down[0] = self.mouse_down[1];
        self.mouse_position[0] = self.mouse_position[1];
        self.key_down[0] = self.key_down[1];
        self.mouse_wheel_delta = [0.0, 0.0];
    }

    pub fn mouse_at(&self, frame_offset: usize) -> [f64; 2] { self.mouse_position[frame_offset] }
    pub fn delta_mouse_at(&self, frame_offset: usize) -> [f64; 2] { 
        [self.mouse_position[frame_offset+1][0] - self.mouse_position[frame_offset][0], 
         self.mouse_position[frame_offset+1][1] - self.mouse_position[frame_offset][1]] 
    }
    pub fn mouse_down_at(&self, button: MouseButton, frame_offset: usize) -> bool {
        self.mouse_down[frame_offset][button_id(button)]
    }
    pub fn mouse_pressed_at(&self, button: MouseButton, frame_offset: usize) -> bool {
        self.mouse_down_at(button, frame_offset+0) && !self.mouse_down_at(button, frame_offset+1)
    }
    pub fn mouse_released_at(&self, button: MouseButton, frame_offset: usize) -> bool {
        !self.mouse_down_at(button, frame_offset+0) && self.mouse_down_at(button, frame_offset+1)
    }
    pub fn key_down_at(&self, key: KeyCode, frame_offset: usize) -> bool {
        self.key_down[frame_offset][key as usize]
    }
    pub fn key_pressed_at(&self, key: KeyCode, frame_offset: usize) -> bool {
        self.key_down_at(key, frame_offset+0) && !self.key_down_at(key, frame_offset+1)
    }
    pub fn key_released_at(&self, key: KeyCode, frame_offset: usize) -> bool {
        !self.key_down_at(key, frame_offset+0) && self.key_down_at(key, frame_offset+1)
    }

    pub fn mouse(&self) -> [f64; 2] { self.mouse_at(0) }
    pub fn delta_mouse(&self) -> [f64; 2] { self.delta_mouse_at(0) }
    pub fn mouse_down(&self, button: MouseButton) -> bool { self.mouse_down_at(button, 0) }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool { self.mouse_pressed_at(button, 0) }
    pub fn mouse_released(&self, button: MouseButton) -> bool { self.mouse_released_at(button, 0) }
    pub fn key_down(&self, key: KeyCode) -> bool { self.key_down_at(key, 0) }
    pub fn key_pressed(&self, key: KeyCode) -> bool { self.key_pressed_at(key, 0) }
    pub fn key_released(&self, key: KeyCode) -> bool { self.key_released_at(key, 0) }
}

pub fn button_id(button: winit::event::MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(id) => id as usize,
    }
}