//! The root view of the main window.

use gpui::{prelude::*, *};

use crate::{
    APP_NAME, theme::ActiveTheme, title_bar::TitleBar, window_frame::client_side_decorations,
};

const DEFAULT_WINDOW_SIZE: Size<Pixels> = size(px(1280.), px(800.));
/// Same minimum as Zed, so the titlebar controls never overlap.
const MIN_WINDOW_SIZE: Size<Pixels> = size(px(360.), px(240.));

pub struct Workspace;

impl Workspace {
    pub fn open_window(cx: &mut App) -> Result<WindowHandle<Self>> {
        cx.open_window(window_options(cx), |_, cx| cx.new(|_| Self))
    }
}

fn window_options(cx: &App) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            DEFAULT_WINDOW_SIZE,
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some(APP_NAME.into()),
            // Hides the native caption; `TitleBar` draws our own.
            appears_transparent: true,
            traffic_light_position: None,
        }),
        // Linux only; Windows and macOS ignore this.
        window_decorations: Some(window_decorations()),
        app_id: Some(APP_NAME.to_lowercase()),
        window_min_size: Some(MIN_WINDOW_SIZE),
        ..Default::default()
    }
}

/// Prefer the desktop's own frame (KDE, wlroots, X11 window managers).
/// Compositors without xdg-decoration support, like GNOME, fall back to
/// client-side, which `client_side_decorations` and `TitleBar` handle.
/// Set `LOOJI_WINDOW_DECORATIONS=client` to force our own frame, the
/// counterpart of Zed's `ZED_WINDOW_DECORATIONS`.
fn window_decorations() -> WindowDecorations {
    match std::env::var("LOOJI_WINDOW_DECORATIONS").as_deref() {
        Ok("client") => WindowDecorations::Client,
        _ => WindowDecorations::Server,
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let content = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.editor_background)
            .text_color(theme.text)
            .when(TitleBar::is_visible(window), |d| {
                d.child(TitleBar::new(APP_NAME))
            })
            .child(
                div()
                    .flex()
                    .flex_1()
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .child("Hello, World!"),
            );

        client_side_decorations(content, window, cx)
    }
}
