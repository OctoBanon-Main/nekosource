use super::NekoView;
use gpui_kit::component::{ActiveTheme, Disableable, Selectable, button::*};
use gpui_kit::{prelude::*, *};

impl NekoView {
    pub(super) fn repo_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(250.0))
            .flex_shrink_0()
            .p_4()
            .gap_4()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child(self.app.t("label-local-repos")),
                    )
                    .child(div().text_xs().child(self.app.repos.len().to_string())),
            )
            .child(
                div()
                    .id("repo-list")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .when(self.app.repos.is_empty(), |container| {
                        container.child(
                            div()
                                .py_4()
                                .text_color(cx.theme().muted_foreground)
                                .child(self.app.t("placeholder-no-repos")),
                        )
                    })
                    .children(self.app.repos.iter().enumerate().map(|(idx, (path, url))| {
                        let branch = self.app.repo_branches.get(idx);
                        let mut branch_label =
                            branch.map(|b| b.name.as_str()).unwrap_or("?").to_string();
                        if let Some((ahead, behind)) = branch.and_then(|b| b.divergence) {
                            branch_label.push_str(&format!("  ↑{ahead} ↓{behind}"));
                        }
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .p_3()
                            .mb_2()
                            .rounded_lg()
                            .border_1()
                            .border_color(cx.theme().border)
                            .bg(if self.app.selected_idx == Some(idx) {
                                cx.theme().muted
                            } else {
                                cx.theme().background
                            })
                            .child(
                                Button::new(("repo", idx))
                                    .ghost()
                                    .w_full()
                                    .label(
                                        std::path::Path::new(path)
                                            .file_name()
                                            .and_then(|name| name.to_str())
                                            .unwrap_or(path)
                                            .to_string(),
                                    )
                                    .selected(self.app.selected_idx == Some(idx))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.app.selected_idx = Some(idx);
                                        this.app.refresh_selected_log();
                                        cx.notify();
                                    })),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(branch_label),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .truncate()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(url.clone()),
                            )
                    })),
            )
            .child(
                Button::new("delete-repo")
                    .label(self.app.t("btn-delete"))
                    .disabled(self.app.selected_idx.is_none() || self.app.is_busy())
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(idx) = this.app.selected_idx.take() {
                            this.app.repos.remove(idx);
                            if idx < this.app.repo_branches.len() {
                                this.app.repo_branches.remove(idx);
                            }
                            this.app.refresh_selected_log();
                            this.app.save_repos();
                            cx.notify();
                        }
                    })),
            )
    }
}
