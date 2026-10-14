use gpui::rgb;

use super::Theme;

impl Theme {
    /// Zed's "Gruvbox Dark Hard" theme (assets/themes/gruvbox/gruvbox.json).
    pub fn gruvbox_dark_hard() -> Self {
        Self {
            title_bar_background: rgb(0x4c4642).into(),
            title_bar_inactive_background: rgb(0x393634).into(),
            editor_background: rgb(0x1d2021).into(),
            border: rgb(0x5b534d).into(),
            text: rgb(0xfbf1c7).into(),
            ghost_element_hover: rgb(0x494340).into(),
            ghost_element_active: rgb(0x5b524c).into(),
        }
    }
}
