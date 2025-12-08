mod templates;
mod wizard_modal;

use std::sync::Arc;

use gpui::{App, actions};
use workspace::{AppState, Workspace};

pub use wizard_modal::{BitcoinAppWizard, Framework, Template};

actions!(bitcoin_app_wizard, [NewBitcoinApp]);

/// Register the Bitcoin app wizard action and modal.
pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    cx.observe_new({
        let app_state = app_state.clone();
        move |workspace: &mut Workspace, _window, _cx| {
            let app_state = app_state.clone();
            workspace.register_action(move |workspace, _: &NewBitcoinApp, window, cx| {
                let workspace_handle = cx.entity().downgrade();
                let app_state = app_state.clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    BitcoinAppWizard::new(workspace_handle, app_state, window, cx)
                });
            });
        }
    })
    .detach();
}

