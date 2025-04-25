use rust_on_rails::prelude::*;

use std::collections::HashMap;

pub struct IconResources(HashMap<&'static str, resources::Image>);

impl IconResources {
    pub fn default(ctx: &mut Context) -> Self {
        let mut icons = HashMap::new();
        let quality = 8.0;

        icons.insert("accounts", ctx.add_svg(&ctx.load_file("icons/accounts.svg").unwrap(), quality));
        icons.insert("add", ctx.add_svg(&ctx.load_file("icons/add.svg").unwrap(), quality));
        icons.insert("app_store", ctx.add_svg(&ctx.load_file("icons/app_store.svg").unwrap(), quality));
        icons.insert("back", ctx.add_svg(&ctx.load_file("icons/back.svg").unwrap(), quality));
        icons.insert("back_arrow", ctx.add_svg(&ctx.load_file("icons/back_arrow.svg").unwrap(), quality));
        icons.insert("back_to", ctx.add_svg(&ctx.load_file("icons/back_arrow.svg").unwrap(), quality));
        icons.insert("backspace", ctx.add_svg(&ctx.load_file("icons/backspace.svg").unwrap(), quality));
        icons.insert("bitcoin", ctx.add_svg(&ctx.load_file("icons/bitcoin.svg").unwrap(), quality));
        icons.insert("camera", ctx.add_svg(&ctx.load_file("icons/camera.svg").unwrap(), quality));
        icons.insert("cancel", ctx.add_svg(&ctx.load_file("icons/cancel.svg").unwrap(), quality));
        icons.insert("capslock", ctx.add_svg(&ctx.load_file("icons/capslock.svg").unwrap(), quality));
        icons.insert("capslock_on", ctx.add_svg(&ctx.load_file("icons/capslock_on.svg").unwrap(), quality));
        icons.insert("checkmark", ctx.add_svg(&ctx.load_file("icons/checkmark.svg").unwrap(), quality));
        icons.insert("close", ctx.add_svg(&ctx.load_file("icons/close.svg").unwrap(), quality));
        icons.insert("copy", ctx.add_svg(&ctx.load_file("icons/copy.svg").unwrap(), quality));
        icons.insert("credential", ctx.add_svg(&ctx.load_file("icons/credential.svg").unwrap(), quality));
        icons.insert("down_arrow", ctx.add_svg(&ctx.load_file("icons/down_arrow.svg").unwrap(), quality));
        icons.insert("delete", ctx.add_svg(&ctx.load_file("icons/delete.svg").unwrap(), quality));
        icons.insert("door", ctx.add_svg(&ctx.load_file("icons/door.svg").unwrap(), quality));
        icons.insert("down", ctx.add_svg(&ctx.load_file("icons/down.svg").unwrap(), quality));
        icons.insert("edit", ctx.add_svg(&ctx.load_file("icons/edit.svg").unwrap(), quality));
        icons.insert("emoji", ctx.add_svg(&ctx.load_file("icons/emoji.svg").unwrap(), quality));
        icons.insert("error", ctx.add_svg(&ctx.load_file("icons/error.svg").unwrap(), quality));
        icons.insert("explore", ctx.add_svg(&ctx.load_file("icons/explore.svg").unwrap(), quality));
        icons.insert("facebook", ctx.add_svg(&ctx.load_file("icons/facebook.svg").unwrap(), quality));
        icons.insert("forward", ctx.add_svg(&ctx.load_file("icons/forward.svg").unwrap(), quality));
        icons.insert("gif", ctx.add_svg(&ctx.load_file("icons/gif.svg").unwrap(), quality));
        icons.insert("group", ctx.add_svg(&ctx.load_file("icons/group.svg").unwrap(), quality));
        icons.insert("heart", ctx.add_svg(&ctx.load_file("icons/heart.svg").unwrap(), quality));
        icons.insert("home", ctx.add_svg(&ctx.load_file("icons/home.svg").unwrap(), quality));
        icons.insert("infinite", ctx.add_svg(&ctx.load_file("icons/infinite.svg").unwrap(), quality));
        icons.insert("info", ctx.add_svg(&ctx.load_file("icons/info.svg").unwrap(), quality));
        icons.insert("instagram", ctx.add_svg(&ctx.load_file("icons/instagram.svg").unwrap(), quality));
        icons.insert("left", ctx.add_svg(&ctx.load_file("icons/left.svg").unwrap(), quality));
        icons.insert("link", ctx.add_svg(&ctx.load_file("icons/link.svg").unwrap(), quality));
        icons.insert("megaphone", ctx.add_svg(&ctx.load_file("icons/megaphone.svg").unwrap(), quality));
        icons.insert("messages", ctx.add_svg(&ctx.load_file("icons/messages.svg").unwrap(), quality));
        icons.insert("microphone", ctx.add_svg(&ctx.load_file("icons/microphone.svg").unwrap(), quality));
        icons.insert("monitor", ctx.add_svg(&ctx.load_file("icons/monitor.svg").unwrap(), quality));
        icons.insert("paste", ctx.add_svg(&ctx.load_file("icons/paste.svg").unwrap(), quality));
        icons.insert("photos", ctx.add_svg(&ctx.load_file("icons/photos.svg").unwrap(), quality));
        icons.insert("play_store", ctx.add_svg(&ctx.load_file("icons/play_store.svg").unwrap(), quality));
        icons.insert("profile", ctx.add_svg(&ctx.load_file("icons/profile.svg").unwrap(), quality));
        icons.insert("qr_code", ctx.add_svg(&ctx.load_file("icons/qr_code.svg").unwrap(), quality));
        icons.insert("radio_filled", ctx.add_svg(&ctx.load_file("icons/radio_filled.svg").unwrap(), quality));
        icons.insert("radio", ctx.add_svg(&ctx.load_file("icons/radio.svg").unwrap(), quality));
        icons.insert("right", ctx.add_svg(&ctx.load_file("icons/right.svg").unwrap(), quality));
        icons.insert("scan", ctx.add_svg(&ctx.load_file("icons/scan.svg").unwrap(), quality));
        icons.insert("search", ctx.add_svg(&ctx.load_file("icons/search.svg").unwrap(), quality));
        icons.insert("send", ctx.add_svg(&ctx.load_file("icons/send.svg").unwrap(), quality));
        icons.insert("settings", ctx.add_svg(&ctx.load_file("icons/settings.svg").unwrap(), quality));
        icons.insert("up", ctx.add_svg(&ctx.load_file("icons/up.svg").unwrap(), quality));
        icons.insert("wallet", ctx.add_svg(&ctx.load_file("icons/wallet.svg").unwrap(), quality));
        icons.insert("warning", ctx.add_svg(&ctx.load_file("icons/warning.svg").unwrap(), quality));
        icons.insert("x", ctx.add_svg(&ctx.load_file("icons/x.svg").unwrap(), quality));

        Self(icons)
    }

    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| panic!("Could not find icon {:?}", name)).clone()
    }

    pub fn add_icon(&mut self, icon_name: &'static str, icon: resources::Image) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.0.entry(icon_name) {
            e.insert(icon);
        } else {
            println!("add_icon(): Icon with name {:?} already exists. Use 'set_icon()' instead.", icon_name);
        }
    }

    pub fn set_icon(&mut self, icon_name: &'static str, icon: resources::Image) {
        if let Some(existing) = self.0.get_mut(&icon_name) {
            *existing = icon; 
        } else {
            println!("set_icon(): Icon with name {:?} doesn't exist. Use 'add_icon()' instead.", icon_name);
        }
    }
}
