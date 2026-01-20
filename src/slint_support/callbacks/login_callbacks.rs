//! Login-related callback wiring for Slint UI.

use crossbeam_channel::Sender;
use slint::ComponentHandle;

use crate::webui::ipc::{ServerNoId, ServerWithId, UiToCore};
use crate::{LoginBridge, MainWindow};

/// Wire all login-related callbacks: login, saved credentials, server management.
pub fn wire_login_callbacks(slint_app: &MainWindow, tx: Sender<UiToCore>) {
    let login_bridge = slint_app.global::<LoginBridge>();

    // Attempt login
    {
        let tx = tx.clone();
        login_bridge.on_attempt_login(move |server_id, username, password, remember| {
            let _ = tx.send(UiToCore::LoginSubmit {
                server_id: server_id as u32,
                username: username.to_string(),
                password: password.to_string(),
                remember,
            });
        });
    }

    // Use saved credentials
    {
        let tx = tx.clone();
        login_bridge.on_use_saved(move |id| {
            let _ = tx.send(UiToCore::LoginUseSaved { id: id.to_string() });
        });
    }

    // Remove saved credentials
    {
        let tx = tx.clone();
        login_bridge.on_remove_saved(move |id| {
            let _ = tx.send(UiToCore::LoginRemoveSaved { id: id.to_string() });
        });
    }

    // Change current server
    {
        let tx = tx.clone();
        login_bridge.on_change_current_server(move |id| {
            let _ = tx.send(UiToCore::ServersChangeCurrent { id: id as u32 });
        });
    }

    // Add server
    {
        let tx = tx.clone();
        login_bridge.on_add_server(move |name, address| {
            let server = ServerNoId {
                name: name.to_string(),
                address: address.to_string(),
            };
            let _ = tx.send(UiToCore::ServersAdd { server });
        });
    }

    // Edit server
    {
        let tx = tx.clone();
        login_bridge.on_edit_server(move |id, name, address| {
            let server = ServerWithId {
                id: id as u32,
                name: name.to_string(),
                address: address.to_string(),
            };
            let _ = tx.send(UiToCore::ServersEdit { server });
        });
    }

    // Remove server
    {
        let tx = tx.clone();
        login_bridge.on_remove_server(move |id| {
            let _ = tx.send(UiToCore::ServersRemove { id: id as u32 });
        });
    }

    // Request snapshot (on MainWindow, not login bridge)
    {
        let tx = tx.clone();
        slint_app.on_request_snapshot(move || {
            let _ = tx.send(UiToCore::RequestSnapshot);
        });
    }
}
