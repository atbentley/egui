//! uis for emigui types.
use crate::{
    containers::show_tooltip,
    label,
    math::*,
    paint::{color::WHITE, PaintCmd, Texture, Triangles, Vertex},
};

impl Texture {
    pub fn ui(&self, ui: &mut crate::Ui) {
        ui.add(label!(
            "Texture size: {} x {} (hover to zoom)",
            self.width,
            self.height
        ));
        let mut size = vec2(self.width as f32, self.height as f32);
        if size.x > ui.available().width() {
            size *= ui.available().width() / size.x;
        }
        let rect = ui.allocate_space(size);
        let top_left = Vertex {
            pos: rect.min,
            uv: (0, 0),
            color: WHITE,
        };
        let bottom_right = Vertex {
            pos: rect.max,
            uv: (self.width as u16 - 1, self.height as u16 - 1),
            color: WHITE,
        };
        let mut triangles = Triangles::default();
        triangles.add_rect(top_left, bottom_right);
        ui.add_paint_cmd(PaintCmd::Triangles(triangles));

        if ui.hovered(rect) {
            show_tooltip(ui.ctx(), |ui| {
                let pos = ui.top_left();
                let zoom_rect = ui.allocate_space(vec2(128.0, 128.0));
                let u = remap_clamp(pos.x, rect.range_x(), 0.0..=self.width as f32 - 1.0).round();
                let v = remap_clamp(pos.y, rect.range_y(), 0.0..=self.height as f32 - 1.0).round();

                let texel_radius = 32.0;
                let u = clamp(u, texel_radius..=self.width as f32 - 1.0 - texel_radius);
                let v = clamp(v, texel_radius..=self.height as f32 - 1.0 - texel_radius);

                let top_left = Vertex {
                    pos: zoom_rect.min,
                    uv: ((u - texel_radius) as u16, (v - texel_radius) as u16),
                    color: WHITE,
                };
                let bottom_right = Vertex {
                    pos: zoom_rect.max,
                    uv: ((u + texel_radius) as u16, (v + texel_radius) as u16),
                    color: WHITE,
                };
                let mut triangles = Triangles::default();
                triangles.add_rect(top_left, bottom_right);
                ui.add_paint_cmd(PaintCmd::Triangles(triangles));
            });
        }
    }
}
