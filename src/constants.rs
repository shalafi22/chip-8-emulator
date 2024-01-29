

    use crate::chip8::Sprite;
    use sdl2::keyboard::Keycode;
    
const ZERO_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0x90, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const ONE_SPRITE: Sprite = Sprite {bytes: [0x20, 0x60, 0x20, 0x20, 0x70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const TWO_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0xF0, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const THREE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const FOUR_SPRITE: Sprite = Sprite {bytes: [0x90, 0x90, 0xF0, 0x10, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const FIVE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const SIX_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const SEVEN_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x10, 0x20, 0x40, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const EIGHT_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x90, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const NINE_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x10, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const A_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x90, 0xF0, 0x90, 0x90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const B_SPRITE: Sprite = Sprite {bytes: [0xE0, 0x90, 0xE0, 0x90, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const C_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0x80, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const D_SPRITE: Sprite = Sprite {bytes: [0xE0, 0x90, 0x90, 0x90, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const E_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x80, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};
const F_SPRITE: Sprite = Sprite {bytes: [0xF0, 0x80, 0xF0, 0x80, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], size: 5};


pub const SPRITE_PRESET: [Sprite; 16] = [ZERO_SPRITE, ONE_SPRITE, TWO_SPRITE, THREE_SPRITE, FOUR_SPRITE, FIVE_SPRITE, SIX_SPRITE, SEVEN_SPRITE,
                                    EIGHT_SPRITE, NINE_SPRITE, A_SPRITE, B_SPRITE, C_SPRITE, D_SPRITE, E_SPRITE, F_SPRITE];

pub const KEYPAD_VALUES: [Keycode; 16] = [
    Keycode::X,
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::Num4,
    Keycode::R,
    Keycode::F,
    Keycode::V
];
