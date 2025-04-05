use rust_on_rails::prelude::*;

use std::collections::HashMap;

pub struct IconResources(HashMap<&'static str, resources::Image>);

impl IconResources {
    pub fn default(ctx: &mut Context) -> Self {
        let mut icons = HashMap::new();
        let quality = 8.0;

        icons.insert("accounts", resources::Image::svg(ctx, &ctx.load_file("icons/accounts.svg").unwrap(), quality));
        icons.insert("add", resources::Image::svg(ctx, &ctx.load_file("icons/add.svg").unwrap(), quality));
        icons.insert("app_store", resources::Image::svg(ctx, &ctx.load_file("icons/app_store.svg").unwrap(), quality));
        icons.insert("back", resources::Image::svg(ctx, &ctx.load_file("icons/back.svg").unwrap(), quality));
        icons.insert("backspace", resources::Image::svg(ctx, &ctx.load_file("icons/backspace.svg").unwrap(), quality));
        icons.insert("bitcoin", resources::Image::svg(ctx, &ctx.load_file("icons/bitcoin.svg").unwrap(), quality));
        icons.insert("camera", resources::Image::svg(ctx, &ctx.load_file("icons/camera.svg").unwrap(), quality));
        icons.insert("cancel", resources::Image::svg(ctx, &ctx.load_file("icons/cancel.svg").unwrap(), quality));
        icons.insert("capslock", resources::Image::svg(ctx, &ctx.load_file("icons/capslock.svg").unwrap(), quality));
        icons.insert("capslock_on", resources::Image::svg(ctx, &ctx.load_file("icons/capslock_on.svg").unwrap(), quality));
        icons.insert("checkmark", resources::Image::svg(ctx, &ctx.load_file("icons/checkmark.svg").unwrap(), quality));
        icons.insert("close", resources::Image::svg(ctx, &ctx.load_file("icons/close.svg").unwrap(), quality));
        icons.insert("copy", resources::Image::svg(ctx, &ctx.load_file("icons/copy.svg").unwrap(), quality));
        icons.insert("credential", resources::Image::svg(ctx, &ctx.load_file("icons/credential.svg").unwrap(), quality));
        icons.insert("delete", resources::Image::svg(ctx, &ctx.load_file("icons/delete.svg").unwrap(), quality));
        icons.insert("door", resources::Image::svg(ctx, &ctx.load_file("icons/door.svg").unwrap(), quality));
        icons.insert("down", resources::Image::svg(ctx, &ctx.load_file("icons/down.svg").unwrap(), quality));
        icons.insert("edit", resources::Image::svg(ctx, &ctx.load_file("icons/edit.svg").unwrap(), quality));
        icons.insert("emoji", resources::Image::svg(ctx, &ctx.load_file("icons/emoji.svg").unwrap(), quality));
        icons.insert("error", resources::Image::svg(ctx, &ctx.load_file("icons/error.svg").unwrap(), quality));
        icons.insert("explore", resources::Image::svg(ctx, &ctx.load_file("icons/explore.svg").unwrap(), quality));
        icons.insert("facebook", resources::Image::svg(ctx, &ctx.load_file("icons/facebook.svg").unwrap(), quality));
        icons.insert("forward", resources::Image::svg(ctx, &ctx.load_file("icons/forward.svg").unwrap(), quality));
        icons.insert("gif", resources::Image::svg(ctx, &ctx.load_file("icons/gif.svg").unwrap(), quality));
        icons.insert("group", resources::Image::svg(ctx, &ctx.load_file("icons/group.svg").unwrap(), quality));
        icons.insert("heart", resources::Image::svg(ctx, &ctx.load_file("icons/heart.svg").unwrap(), quality));
        icons.insert("home", resources::Image::svg(ctx, &ctx.load_file("icons/home.svg").unwrap(), quality));
        icons.insert("infinite", resources::Image::svg(ctx, &ctx.load_file("icons/infinite.svg").unwrap(), quality));
        icons.insert("info", resources::Image::svg(ctx, &ctx.load_file("icons/info.svg").unwrap(), quality));
        icons.insert("instagram", resources::Image::svg(ctx, &ctx.load_file("icons/instagram.svg").unwrap(), quality));
        icons.insert("left", resources::Image::svg(ctx, &ctx.load_file("icons/left.svg").unwrap(), quality));
        icons.insert("link", resources::Image::svg(ctx, &ctx.load_file("icons/link.svg").unwrap(), quality));
        icons.insert("megaphone", resources::Image::svg(ctx, &ctx.load_file("icons/megaphone.svg").unwrap(), quality));
        icons.insert("messages", resources::Image::svg(ctx, &ctx.load_file("icons/messages.svg").unwrap(), quality));
        icons.insert("microphone", resources::Image::svg(ctx, &ctx.load_file("icons/microphone.svg").unwrap(), quality));
        icons.insert("monitor", resources::Image::svg(ctx, &ctx.load_file("icons/monitor.svg").unwrap(), quality));
        icons.insert("paste", resources::Image::svg(ctx, &ctx.load_file("icons/paste.svg").unwrap(), quality));
        icons.insert("photos", resources::Image::svg(ctx, &ctx.load_file("icons/photos.svg").unwrap(), quality));
        icons.insert("play_store", resources::Image::svg(ctx, &ctx.load_file("icons/play_store.svg").unwrap(), quality));
        icons.insert("profile", resources::Image::svg(ctx, &ctx.load_file("icons/profile.svg").unwrap(), quality));
        icons.insert("qr_code", resources::Image::svg(ctx, &ctx.load_file("icons/qr_code.svg").unwrap(), quality));
        icons.insert("radio_filled", resources::Image::svg(ctx, &ctx.load_file("icons/radio_filled.svg").unwrap(), quality));
        icons.insert("radio", resources::Image::svg(ctx, &ctx.load_file("icons/radio.svg").unwrap(), quality));
        icons.insert("right", resources::Image::svg(ctx, &ctx.load_file("icons/right.svg").unwrap(), quality));
        icons.insert("scan", resources::Image::svg(ctx, &ctx.load_file("icons/scan.svg").unwrap(), quality));
        icons.insert("search", resources::Image::svg(ctx, &ctx.load_file("icons/search.svg").unwrap(), quality));
        icons.insert("send", resources::Image::svg(ctx, &ctx.load_file("icons/send.svg").unwrap(), quality));
        icons.insert("settings", resources::Image::svg(ctx, &ctx.load_file("icons/settings.svg").unwrap(), quality));
        icons.insert("up", resources::Image::svg(ctx, &ctx.load_file("icons/up.svg").unwrap(), quality));
        icons.insert("wallet", resources::Image::svg(ctx, &ctx.load_file("icons/wallet.svg").unwrap(), quality));
        icons.insert("warning", resources::Image::svg(ctx, &ctx.load_file("icons/warning.svg").unwrap(), quality));
        icons.insert("x", resources::Image::svg(ctx, &ctx.load_file("icons/x.svg").unwrap(), quality));

        Self(icons)
    }

    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).expect(&format!("Could not find icon {:?}", name)).clone()
    }

    pub fn add_icon(&mut self, icon_name: &'static str, icon: resources::Image) {
        if self.0.get(&icon_name).is_some() {
            println!("add_icon(): Icon with name {:?} already exists. Use 'set_icon()' instead.", icon_name);
        } else {
            self.0.insert(icon_name, icon);
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
