//! Built-in pet catalog ported from the Codex App avatar catalog.

use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use std::fs;
#[cfg(test)]
use std::io::Cursor;
#[cfg(test)]
use std::sync::LazyLock;
use std::sync::OnceLock;

pub(super) const DEFAULT_FRAME_WIDTH: u32 = 192;
pub(super) const DEFAULT_FRAME_HEIGHT: u32 = 208;
pub(super) const DEFAULT_FRAME_COLUMNS: u32 = 8;
pub(super) const DEFAULT_FRAME_ROWS: u32 = 9;
pub(super) const SPRITESHEET_WIDTH: u32 = DEFAULT_FRAME_WIDTH * DEFAULT_FRAME_COLUMNS;
pub(super) const SPRITESHEET_HEIGHT: u32 = DEFAULT_FRAME_HEIGHT * DEFAULT_FRAME_ROWS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BuiltinPet {
    pub(super) id: &'static str,
    pub(super) display_name: &'static str,
    pub(super) description: &'static str,
    pub(super) spritesheet_file: &'static str,
}

/// 内置宠物目录。
///
/// 用 `OnceLock` + 函数而不是 `const` 表：描述文案要过 `tr()`，而 `tr` 不是 `const fn`；
/// 表在首次使用时构建，与 footer 的快捷键表同一模式。
pub(super) fn builtin_pets() -> &'static [BuiltinPet] {
    static PETS: OnceLock<Vec<BuiltinPet>> = OnceLock::new();
    PETS.get_or_init(|| {
        vec![
            BuiltinPet {
                id: "codex",
                display_name: "Codex",
                description: tr(current(), "The original Codex companion"),
                spritesheet_file: "codex-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "dewey",
                display_name: "Dewey",
                description: tr(current(), "A tidy duck for calm workspace days"),
                spritesheet_file: "dewey-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "fireball",
                display_name: "Fireball",
                description: tr(current(), "Hot path energy for fast iteration"),
                spritesheet_file: "fireball-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "rocky",
                display_name: "Rocky",
                description: tr(current(), "A steady rock when the diff gets large"),
                spritesheet_file: "rocky-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "seedy",
                display_name: "Seedy",
                description: tr(current(), "Small green shoots for new ideas"),
                spritesheet_file: "seedy-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "stacky",
                display_name: "Stacky",
                description: tr(current(), "A balanced stack for deep work"),
                spritesheet_file: "stacky-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "bsod",
                display_name: "BSOD",
                description: tr(current(), "A tiny blue-screen gremlin"),
                spritesheet_file: "bsod-spritesheet-v4.webp",
            },
            BuiltinPet {
                id: "null-signal",
                display_name: "Null Signal",
                description: tr(current(), "Quiet signal from the void"),
                spritesheet_file: "null-signal-spritesheet-v4.webp",
            },
        ]
    })
}

pub(super) fn builtin_pet(id: &str) -> Option<BuiltinPet> {
    builtin_pets().iter().copied().find(|pet| pet.id == id)
}

#[cfg(test)]
pub(super) fn write_test_spritesheet(path: &std::path::Path) {
    static TEST_SPRITESHEET: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let image = image::RgbaImage::new(SPRITESHEET_WIDTH, SPRITESHEET_HEIGHT);
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::WebP)
            .unwrap();
        bytes
    });
    fs::write(path, TEST_SPRITESHEET.as_slice()).unwrap();
}
