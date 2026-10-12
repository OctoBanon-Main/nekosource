mod app;

use app::NekoApp;
use gpui_kit::{AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size};

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(780.0)),
                    cx,
                ))),
                window_min_size: Some(size(px(1000.0), px(650.0))),
                titlebar: Some(TitlebarOptions {
                    title: Some("NekoSource".into()),
                    ..Default::default()
                }),
                is_resizable: true,
                ..Default::default()
            };
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| NekoApp::new(window, cx))
            })
            .expect("Failed to open NekoSource window");
            cx.activate(true);
        });
}
