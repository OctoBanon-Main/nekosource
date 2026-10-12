use crate::app::{state::Status, ui::NekoView};
use gpui_kit::component::{WindowExt, button::*, spinner::Spinner};
use gpui_kit::{prelude::*, *};

impl NekoView {
    pub(in crate::app::ui) fn show_status_dialog(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (title, message, loading) = match &self.app.status {
            Status::Idle => return,
            Status::Loading(message) => (self.app.t("status-loading"), message.clone(), true),
            Status::Success(message) => (self.app.t("status-success"), message.clone(), false),
            Status::Error(message) => (self.app.t("status-error"), message.clone(), false),
        };
        let view = cx.entity();
        window.open_dialog(cx, move |dialog, _, cx| {
            view.update(cx, |this, cx| {
                dialog
                    .title(title.clone())
                    .overlay_closable(false)
                    .close_button(!loading)
                    .keyboard(!loading)
                    .on_close(cx.listener(|this, _, _, cx| {
                        this.app.status = Status::Idle;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(loading, |div| div.child(Spinner::new()))
                            .child(message.clone()),
                    )
                    .when(!loading, |dialog| {
                        dialog.footer(
                            Button::new("close-status")
                                .label(this.app.t("btn-close"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.app.status = Status::Idle;
                                    window.close_dialog(cx);
                                    cx.notify();
                                })),
                        )
                    })
            })
        });
    }
}
