use super::{NekoView, workers};
use gpui_kit::component::{ActiveTheme, Disableable, button::*, input::Input};
use gpui_kit::{prelude::*, *};

impl NekoView {
    pub(super) fn toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_5()
                    .py_3()
                    .child(
                        div()
                            .id("app-title")
                            .test_support()
                            .flex_1()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .child("🐱 NekoSource"),
                    )
                    .child(
                        Button::new("refresh")
                            .ghost()
                            .label(self.app.t("btn-refresh"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.app.refresh_all_branches();
                                this.app.refresh_selected_log();
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("settings")
                            .ghost()
                            .label(self.app.t("btn-settings"))
                            .disabled(self.app.is_busy())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_settings_dialog(window, cx)
                            })),
                    )
                    .child(
                        Button::new("about")
                            .ghost()
                            .label(self.app.t("btn-about"))
                            .disabled(self.app.is_busy())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_about_dialog(window, cx)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_5()
                    .py_3()
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(self.app.t("label-url")),
                    )
                    .child(
                        div()
                            .id("clone-url")
                            .test_support()
                            .flex_1()
                            .min_w_0()
                            .child(
                                Input::new(&self.clone_url)
                                    .disabled(self.app.is_busy())
                                    .aria_label(self.app.t("label-url")),
                            ),
                    )
                    .child(
                        div().id("clone-branch").test_support().w(px(140.0)).child(
                            Input::new(&self.clone_branch)
                                .disabled(self.app.is_busy())
                                .aria_label(self.app.t("hint-branch")),
                        ),
                    )
                    .child(
                        Button::new("clone")
                            .primary()
                            .label(self.app.t("btn-clone"))
                            .disabled(
                                self.app.is_busy() || self.app.clone_url_input.trim().is_empty(),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                if !this.app.is_busy() {
                                    workers::spawn_clone(&mut this.app);
                                    if this.app.is_busy() {
                                        this.watch_worker(window, cx);
                                    }
                                }
                            })),
                    ),
            )
    }
}
