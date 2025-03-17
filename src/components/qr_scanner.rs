use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use qrcode::{QrCode, Color};

pub struct QRCodeScanner();

impl ComponentBuilder for QRCodeScanner {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<(Box<dyn ComponentBuilder>, Vec2)> = vec![];

        let camera_disabled = true; // User needs to enable camera access
        let waiting = true; // Camera is being loaded up

        let bg = if camera_disabled { COLORS.background.secondary } else { COLORS.shades.transparent };

        children.push(RoundedRectangle(236, 236, 8, bg, Some(COLORS.shades.lighten2, 4)));

        if waiting || camera_disabled {
            let (icon, msg) = match waiting {
                true => (Icon::Camera, "Accessing device camera."), // Wait for camera
                false => (Icon::Settings, "Enable camera in settings.") // Disabled camera
            };

            children.push(Column(ZERO, 4, Align::Center, vec![
                icon.build(COLORS.text.secondary, 48), // Icon
                Text::secondary(ctx, msg, TextSize::sm()) // Help Message
            ]))
        }

        Stack(ZERO, Align::Center, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}