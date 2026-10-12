use crate::app::{git, gpg, state::Status, ui::NekoView};
use crate::app::{
    language::{Language, Locale},
    state::NekoApp,
};
use gpui_kit::component::{
    ActiveTheme, IndexPath, Selectable, Theme, ThemeMode, WindowExt,
    button::*,
    checkbox::Checkbox,
    input::Input,
    select::{SearchableVec, Select, SelectEvent, SelectState},
};
use gpui_kit::{prelude::*, *};

const LANGUAGES: [(Language, &str); 4] = [
    (Language::En, "English"),
    (Language::Ru, "Русский"),
    (Language::Uk, "Українська"),
    (Language::Be, "Беларуская"),
];

impl NekoView {
    pub(in crate::app::ui) fn language_select(
        app: &NekoApp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<SelectState<SearchableVec<&'static str>>> {
        let selected = LANGUAGES
            .iter()
            .position(|(lang, _)| *lang == app.current_lang)
            .unwrap();
        let select = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(LANGUAGES.iter().map(|(_, name)| *name).collect::<Vec<_>>()),
                Some(IndexPath::new(selected)),
                window,
                cx,
            )
        });
        cx.subscribe_in(&select, window, |this, state, event, window, cx| {
            if let SelectEvent::Confirm(Some(_)) = event
                && let Some(index) = state.read(cx).selected_index(cx)
            {
                this.app.current_lang = LANGUAGES[index.row].0;
                this.app.locale = Locale::new(this.app.current_lang);
                this.clone_branch.update(cx, |input, cx| {
                    input.set_placeholder(this.app.t("hint-branch"), window, cx);
                });
                cx.notify();
            }
        })
        .detach();
        select
    }

    pub(in crate::app::ui) fn show_settings_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.app.show_settings = true;
        let view = cx.entity();
        window.open_dialog(cx, move |dialog, _, cx| {
            view.update(cx, |this, cx| {
                dialog
                    .title(this.app.t("settings-title"))
                    .w(px(540.0))
                    .overlay_closable(false)
                    .on_close(cx.listener(|this, _, _, cx| {
                        this.app.show_settings = false;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(this.app.t("label-language"))
                            .child(
                                Select::new(&this.language_select)
                                    .id("language")
                                    .accessibility_label(this.app.t("label-language")),
                            )
                            .child(
                                Checkbox::new("dark-theme")
                                    .label(this.app.t("label-dark-theme"))
                                    .checked(cx.theme().mode.is_dark())
                                    .on_click(cx.listener(|_, dark, window, cx| {
                                        Theme::change(
                                            if *dark {
                                                ThemeMode::Dark
                                            } else {
                                                ThemeMode::Light
                                            },
                                            Some(window),
                                            cx,
                                        );
                                    })),
                            )
                            .child(this.app.t("label-username"))
                            .child(
                                Input::new(&this.username).aria_label(this.app.t("label-username")),
                            )
                            .child(this.app.t("label-email"))
                            .child(Input::new(&this.email).aria_label(this.app.t("label-email")))
                            .child(
                                Checkbox::new("gpg-enabled")
                                    .label(this.app.t("label-gpg"))
                                    .checked(this.app.enable_gpg)
                                    .on_click(cx.listener(|this, enabled, _, cx| {
                                        this.app.enable_gpg = *enabled;
                                        cx.notify();
                                    })),
                            )
                            .when(this.app.enable_gpg, |container| {
                                container
                                    .child(this.app.t("label-gpg-id"))
                                    .child(
                                        div()
                                            .id("gpg-keys")
                                            .max_h(px(180.0))
                                            .overflow_y_scroll()
                                            .when(this.app.gpg_keys.is_empty(), |div| {
                                                div.child(this.app.t("gpg-no-keys"))
                                            })
                                            .children(this.app.gpg_keys.iter().map(
                                                |(id, label)| {
                                                    let id = id.clone();
                                                    Button::new(SharedString::from(id.clone()))
                                                        .label(label.clone())
                                                        .selected(this.app.gpg_key_id == id)
                                                        .on_click(cx.listener(
                                                            move |this, _, _, cx| {
                                                                this.app.gpg_key_id = id.clone();
                                                                cx.notify();
                                                            },
                                                        ))
                                                },
                                            )),
                                    )
                                    .child(
                                        Button::new("reload-gpg")
                                            .label(this.app.t("gpg-refresh-keys"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.app.gpg_keys = gpg::list_keys();
                                                if !this
                                                    .app
                                                    .gpg_keys
                                                    .iter()
                                                    .any(|(id, _)| id == &this.app.gpg_key_id)
                                                {
                                                    this.app.gpg_key_id = this
                                                        .app
                                                        .gpg_keys
                                                        .first()
                                                        .map(|(id, _)| id.clone())
                                                        .unwrap_or_default();
                                                }
                                                cx.notify();
                                            })),
                                    )
                            }),
                    )
                    .footer(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("save-settings")
                                    .primary()
                                    .label(this.app.t("btn-save"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.app.username =
                                            this.username.read(cx).value().to_string();
                                        this.app.email = this.email.read(cx).value().to_string();
                                        let result = git::write_user_config(
                                            &this.app.username,
                                            &this.app.email,
                                        );
                                        this.app.show_settings = false;
                                        window.close_dialog(cx);
                                        if let Err(error) = result {
                                            this.app.status =
                                                Status::Error(error.localize(&this.app.locale));
                                            this.show_status_dialog(window, cx);
                                        }
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("close-settings")
                                    .label(this.app.t("btn-close"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.app.show_settings = false;
                                        window.close_dialog(cx);
                                        cx.notify();
                                    })),
                            ),
                    )
            })
        });
    }
}
