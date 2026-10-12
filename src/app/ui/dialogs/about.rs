use crate::app::ui::NekoView;
use gpui_kit::component::{ActiveTheme, WindowExt, button::*};
use gpui_kit::{prelude::*, *};

impl NekoView {
    pub(in crate::app::ui) fn show_about_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.app.show_about = true;
        let view = cx.entity();
        window.open_dialog(cx, move |dialog, _, cx| {
            view.update(cx, |this, cx| {
                dialog
                    .title(this.app.t("about-title"))
                    .w(px(540.0))
                    .on_close(cx.listener(|this, _, _, cx| {
                        this.app.show_about = false;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(div().text_xl().child("🐱 NekoSource"))
                            .child(this.app.t("about-description"))
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(this.app.t("about-copyright")),
                            )
                            .child(this.app.t("about-development"))
                            .children(
                                [
                                    (
                                        "OctoBanon",
                                        "https://github.com/OctoBanon-Main",
                                        "contributor-octobanon",
                                    ),
                                    (
                                        "ShadowCj",
                                        "https://github.com/bluzetX",
                                        "contributor-shadowcj",
                                    ),
                                    (
                                        "xelframe",
                                        "https://github.com/xelframe",
                                        "contributor-xelframe",
                                    ),
                                ]
                                .into_iter()
                                .map(|(name, url, role)| {
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            Button::new(name)
                                                .link()
                                                .label(name)
                                                .on_click(move |_, _, cx| cx.open_url(url)),
                                        )
                                        .child(this.app.t(role))
                                }),
                            ),
                    )
                    .footer(
                        Button::new("close-about")
                            .label(this.app.t("btn-close"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.app.show_about = false;
                                window.close_dialog(cx);
                                cx.notify();
                            })),
                    )
            })
        });
    }
}
