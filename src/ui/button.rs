use macroquad::prelude::*;


pub struct ButtonHandler<T, R> {
    key: KeyCode,
    holdable: bool,

    hold_reset_required: bool,
    is_held: bool,
    held_frames: usize,
    hold_delay: usize,
    hold_rate: usize,

    action: fn(&mut T) -> R
}

impl<T, R> ButtonHandler<T, R> {
    pub fn pressable(key: KeyCode, action: fn(&mut T) -> R) -> Self {
        Self {
            key,
            action,

            holdable: false,
            hold_reset_required: true,
            is_held: false,
            held_frames: 0,
            hold_delay: 0,
            hold_rate: 0
        }
    }

    pub fn holdable(key: KeyCode, hold_delay: usize, hold_rate: usize, action: fn(&mut T) -> R) -> Self {
        Self {
            key,
            action,
        
            holdable: true,
            hold_reset_required: false,
            is_held: false,
            held_frames: 0,
            hold_delay,
            hold_rate
        }
    }

    pub fn reset_hold(&mut self) {
        self.is_held = false;
        self.hold_reset_required = true;
    }

    pub fn update(&mut self, receiver: &mut T) -> Option<R> {
        if is_key_down(self.key) {
            if self.is_held {
                if self.holdable {
                    self.held_frames -= 1;
                    if self.held_frames == 0 {
                        self.held_frames = self.hold_rate;
                        return Some((self.action)(receiver));
                    }    
                }
            } else if !self.hold_reset_required {
                self.held_frames = self.hold_delay;
                self.is_held = true;
                return Some((self.action)(receiver));
            }
        } else {
            self.is_held = false;
            self.hold_reset_required = false;
        }

        None
    }
}