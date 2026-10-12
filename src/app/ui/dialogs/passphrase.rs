use crate::app::ui::NekoView;
use gpui_kit::component::{WindowExt, button::*, input::Input};
use gpui_kit::{prelude::*, *};

impl NekoView {
    pub(in crate::app::ui) fn show_passphrase_dialog(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let view = cx.entity();
        window.open_dialog(cx, move |dialog, _, cx| {
            view.update(cx, |this, cx| {
                dialog
                    .title(this.app.t("passphrase-title"))
                    .overlay_closable(false)
                    .close_button(false)
                    .keyboard(false)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(this.app.t("passphrase-prompt"))
                            .child(this.app.passphrase_key_path.clone())
                            .child(this.app.t("passphrase-hint"))
                            .child(
                                Input::new(&this.passphrase)
                                    .aria_label(this.app.t("passphrase-prompt")),
                            ),
                    )
                    .footer(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("submit-passphrase")
                                    .primary()
                                    .label(this.app.t("btn-ok"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.submit_passphrase(true, window, cx)
                                    })),
                            )
                            .child(
                                Button::new("cancel-passphrase")
                                    .label(this.app.t("btn-cancel"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.submit_passphrase(false, window, cx)
                                    })),
                            ),
                    )
            })
        });
        self.passphrase
            .update(cx, |input, cx| input.focus(window, cx));
    }

    pub(in crate::app::ui) fn submit_passphrase(
        &mut self,
        submitted: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(sender) = self.app.passphrase_sender.take() {
            let value = submitted.then(|| self.passphrase.read(cx).value().to_string());
            sender.send(value).ok();
        }
        self.app.passphrase_input.clear();
        self.app.passphrase_key_path.clear();
        self.app.show_passphrase_dialog = false;
        self.passphrase
            .update(cx, |input, cx| input.set_value("", window, cx));
        window.close_dialog(cx);
        self.show_status_dialog(window, cx);
        cx.notify();
    }
}
