use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

pub type ImgRawType = ImageRaw<'static, BinaryColor>;

// 'game-over', WxH Pixel = 100 x 7 px
const SPRITE_GAME_OVER: [u8; 91] = [
    0x7c, 0x03, 0x80, 0x66, 0x07, 0xf0, 0x00, 0x7c, 0x06, 0x60, 0x7f, 0x03, 0xe0, 0x7c, 0x07, 0xc0,
    0x7e, 0x07, 0xf0, 0x00, 0xfe, 0x06, 0x60, 0x7f, 0x03, 0xf0, 0xc0, 0x0c, 0xe0, 0x7e, 0x07, 0x00,
    0x00, 0xce, 0x06, 0x60, 0x70, 0x03, 0x30, 0xdc, 0x0c, 0xe0, 0x7e, 0x07, 0xe0, 0x00, 0xce, 0x07,
    0xe0, 0x7e, 0x03, 0x70, 0xcc, 0x0f, 0xe0, 0x7e, 0x07, 0x00, 0x00, 0xce, 0x03, 0xc0, 0x70, 0x03,
    0xe0, 0x7c, 0x0c, 0xe0, 0x66, 0x07, 0xf0, 0x00, 0xfe, 0x03, 0xc0, 0x7f, 0x03, 0x70, 0x7c, 0x0c,
    0xe0, 0x66, 0x07, 0xf0, 0x00, 0x7c, 0x01, 0x80, 0x7f, 0x03, 0x70,
];
pub const RAW_GAME_OVER: ImgRawType = ImageRaw::new(&SPRITE_GAME_OVER, 100);

// 'bird', WxH Pixel = 12 x 8 px
const SPRITE_BIRD: [u8; 16] = [
    0x1f, 0x80, 0x22, 0x40, 0x44, 0xa0, 0x74, 0x20, 0x8b, 0xf0, 0x4a, 0x10, 0x31, 0xe0, 0x1f,
    0x80,
    // 0xe0, 0x70, 0xdd, 0xb0, 0xbb, 0x50, 0x8b, 0xd0, 0x74, 0x00, 0xb5, 0xe0, 0xce, 0x10, 0xe0, 0x70,
];
pub const RAW_BIRD: ImgRawType = ImageRaw::new(&SPRITE_BIRD, 12);

// 'pipe', WxH Pixel = 14 x 60 px
const SPRITE_PIPE_BOTTOM: [u8; 120] = [
    0x00, 0x00, 0x1f, 0xf8, 0x1f, 0xf8, 0x1f, 0xf8, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
];

// 'pipe', WxH Pixel = 14 x 60 px
const SPRITE_PIPE_TOP: [u8; 120] = [
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0,
    0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00,
    0x1f, 0xf8, 0x1f, 0xf8, 0x1f, 0xf8, 0x00, 0x00,
];

pub const RAW_PIPE_BOTTOM: ImgRawType = ImageRaw::new(&SPRITE_PIPE_BOTTOM, 14);
pub const RAW_PIPE_TOP: ImgRawType = ImageRaw::new(&SPRITE_PIPE_TOP, 14);
