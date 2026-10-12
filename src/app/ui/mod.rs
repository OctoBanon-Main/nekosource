mod commit_panel;
mod dialogs;
mod repo_list;
mod toolbar;
mod workers;

use super::state::NekoApp;
use gpui_kit::component::{
    ActiveTheme, WindowExt,
    input::{InputEvent, InputState, TextareaState},
    select::{SearchableVec, SelectState},
};
use gpui_kit::{prelude::*, *};
use std::time::Duration;

pub struct NekoView {
    app: NekoApp,
    clone_url: Entity<InputState>,
    clone_branch: Entity<InputState>,
    commit_message: Entity<TextareaState>,
    username: Entity<InputState>,
    email: Entity<InputState>,
    passphrase: Entity<InputState>,
    language_select: Entity<SelectState<SearchableVec<&'static str>>>,
    worker_task: Option<Task<()>>,
    _auto_refresh_task: Task<()>,
}

impl NekoView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let app = NekoApp::new();
        let language_select = Self::language_select(&app, window, cx);
        let clone_url = Self::input(
            app.clone_url_input.clone(),
            false,
            |a| &mut a.clone_url_input,
            window,
            cx,
        );
        clone_url.update(cx, |input, cx| input.set_placeholder("URL", window, cx));
        let clone_branch = Self::input(
            app.clone_branch_input.clone(),
            false,
            |a| &mut a.clone_branch_input,
            window,
            cx,
        );
        clone_branch.update(cx, |input, cx| {
            input.set_placeholder(app.t("hint-branch"), window, cx)
        });
        let username = Self::input(app.username.clone(), false, |a| &mut a.username, window, cx);
        let email = Self::input(app.email.clone(), false, |a| &mut a.email, window, cx);
        let passphrase = Self::input(String::new(), true, |a| &mut a.passphrase_input, window, cx);
        cx.subscribe_in(&passphrase, window, |this, _, event, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) && this.app.show_passphrase_dialog {
                this.submit_passphrase(true, window, cx);
            }
        })
        .detach();
        let commit_message = cx.new(|cx| TextareaState::new(window, cx).rows(3));
        cx.subscribe(&commit_message, |this, state, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.app.commit_msg_input = state.read(cx).value().to_string();
                cx.notify();
            }
        })
        .detach();
        Self {
            app,
            clone_url,
            clone_branch,
            commit_message,
            username,
            email,
            passphrase,
            language_select,
            worker_task: None,
            _auto_refresh_task: Self::auto_refresh(cx),
        }
    }

    fn auto_refresh(cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async |view, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let (repos, selected_idx) = match view.update(cx, |this, _| {
                    (!this.app.is_busy()).then(|| (this.app.repos.clone(), this.app.selected_idx))
                }) {
                    Ok(Some(request)) => request,
                    Ok(None) => continue,
                    Err(_) => break,
                };
                let (repos, branches, commits, changes) = cx
                    .background_executor()
                    .spawn(async move {
                        let branches = repos
                            .iter()
                            .map(|(path, _)| super::git::load_branch(path))
                            .collect();
                        let (commits, changes) = selected_idx
                            .and_then(|idx| repos.get(idx))
                            .map(|(path, _)| {
                                (
                                    super::git::load_commits(path, 25),
                                    super::git::load_changed_files(path),
                                )
                            })
                            .unwrap_or_default();
                        (repos, branches, commits, changes)
                    })
                    .await;
                if view
                    .update(cx, |this, cx| {
                        if this.app.is_busy()
                            || this.app.repos != repos
                            || this.app.selected_idx != selected_idx
                        {
                            return;
                        }
                        if this.app.repo_branches != branches
                            || this.app.selected_commits != commits
                            || this.app.selected_changes != changes
                        {
                            this.app.repo_branches = branches;
                            this.app.selected_commits = commits;
                            this.app.selected_changes = changes;
                            this.app.log_loaded_for = selected_idx;
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
    }

    fn input(
        value: String,
        masked: bool,
        field: fn(&mut NekoApp) -> &mut String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<InputState> {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(value)
                .masked(masked)
        });
        cx.subscribe(&input, move |this, state, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                *field(&mut this.app) = state.read(cx).value().to_string();
                cx.notify();
            }
        })
        .detach();
        input
    }

    fn watch_worker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self._auto_refresh_task = Self::auto_refresh(cx);
        self.show_status_dialog(window, cx);
        self.worker_task = Some(cx.spawn_in(window, async |view, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                let running = view.update_in(cx, |this, window, cx| {
                    if workers::poll_worker(&mut this.app) {
                        if this.app.selected_idx != this.app.log_loaded_for {
                            this.app.refresh_selected_log();
                        }
                        if this.app.show_passphrase_dialog {
                            window.close_dialog(cx);
                            this.show_passphrase_dialog(window, cx);
                        } else {
                            if this.app.worker_rx.is_none() {
                                this.clone_url.update(cx, |input, cx| {
                                    input.set_value(this.app.clone_url_input.clone(), window, cx)
                                });
                                this.clone_branch.update(cx, |input, cx| {
                                    input.set_value(this.app.clone_branch_input.clone(), window, cx)
                                });
                                this.app.refresh_all_branches();
                            }
                            window.close_dialog(cx);
                            this.show_status_dialog(window, cx);
                        }
                        cx.notify();
                    }
                    this.app.worker_rx.is_some()
                });
                if !matches!(running, Ok(true)) {
                    break;
                }
            }
        }));
        cx.notify();
    }
}

impl Render for NekoView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .text_sm()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.toolbar(cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .child(self.repo_list(cx))
                    .child(self.commit_panel(cx)),
            )
    }
}
