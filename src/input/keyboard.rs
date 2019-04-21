use crate::sys;
use crate::Ui;

/// A key identifier
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    Tab = sys::ImGuiKey_Tab,
    LeftArrow = sys::ImGuiKey_LeftArrow,
    RightArrow = sys::ImGuiKey_RightArrow,
    UpArrow = sys::ImGuiKey_UpArrow,
    DownArrow = sys::ImGuiKey_DownArrow,
    PageUp = sys::ImGuiKey_PageUp,
    PageDown = sys::ImGuiKey_PageDown,
    Home = sys::ImGuiKey_Home,
    End = sys::ImGuiKey_End,
    Insert = sys::ImGuiKey_Insert,
    Delete = sys::ImGuiKey_Delete,
    Backspace = sys::ImGuiKey_Backspace,
    Space = sys::ImGuiKey_Space,
    Enter = sys::ImGuiKey_Enter,
    Escape = sys::ImGuiKey_Escape,
    A = sys::ImGuiKey_A,
    C = sys::ImGuiKey_C,
    V = sys::ImGuiKey_V,
    X = sys::ImGuiKey_X,
    Y = sys::ImGuiKey_Y,
    Z = sys::ImGuiKey_Z,
}

impl Key {
    /// All possible `Key` variants
    pub const VARIANTS: [Key; Key::COUNT] = [
        Key::Tab,
        Key::LeftArrow,
        Key::RightArrow,
        Key::UpArrow,
        Key::DownArrow,
        Key::PageUp,
        Key::PageDown,
        Key::Home,
        Key::End,
        Key::Insert,
        Key::Delete,
        Key::Backspace,
        Key::Space,
        Key::Enter,
        Key::Escape,
        Key::A,
        Key::C,
        Key::V,
        Key::X,
        Key::Y,
        Key::Z,
    ];
    /// Total count of `Key` variants
    pub const COUNT: usize = sys::ImGuiKey_COUNT as usize;
}

#[test]
fn test_key_variants() {
    for (idx, &value) in Key::VARIANTS.iter().enumerate() {
        assert_eq!(idx, value as usize);
    }
}

impl<'ui> Ui<'ui> {
    /// Returns the key index of the given key identifier.
    ///
    /// Equivalent to indexing the Io struct `key_map` field: `ui.io().key_map[key]`
    pub fn key_index(&self, key: Key) -> u32 {
        unsafe { sys::igGetKeyIndex(key as i32) as u32 }
    }
    /// Returns true if the key is being held.
    ///
    /// Equivalent to indexing the Io struct `keys_down` field: `ui.io().keys_down[key_index]`
    pub fn is_key_down(&self, key_index: u32) -> bool {
        unsafe { sys::igIsKeyDown(key_index as i32) }
    }
    /// Returns true if the key was pressed (went from !down to down).
    ///
    /// Affected by key repeat settings (`io.key_repeat_delay`, `io.key_repeat_rate`)
    pub fn is_key_pressed(&self, key_index: u32) -> bool {
        unsafe { sys::igIsKeyPressed(key_index as i32, true) }
    }
    /// Returns a count of key presses using the given repeat rate/delay settings.
    ///
    /// Usually returns 0 or 1, but might be >1 if `rate` is small enough that `io.delta_time` >
    /// `rate`.
    pub fn key_pressed_amount(&self, key_index: u32, repeat_delay: f32, rate: f32) -> u32 {
        unsafe { sys::igGetKeyPressedAmount(key_index as i32, repeat_delay, rate) as u32 }
    }
}
