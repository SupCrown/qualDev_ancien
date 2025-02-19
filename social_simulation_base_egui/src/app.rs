use egui::{Sense, Vec2};
use rand::Rng;

pub struct SocialSimulationApp {
    person_positions: Vec<(f32, f32)>,
    person_speeds: Vec<(f32, f32)>,
    person_radius: f32,
    steps_per_frame: u32,
}

impl SocialSimulationApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut entities_positions = Vec::new();
        let mut entities_speeds = Vec::new();

        let mut rng = rand::thread_rng();
        for _ in 0..200 {
            let position = (rng.gen_range(0..800) as f32, rng.gen_range(0..600) as f32);
            let speed = (
                rng.gen_range(-1.0..=1.0) as f32,
                rng.gen_range(-1.0..=1.0) as f32,
            );
            entities_positions.push(position);
            entities_speeds.push(speed);
        }

        SocialSimulationApp {
            person_positions: entities_positions,
            person_speeds: entities_speeds,
            person_radius: 3.0,
            steps_per_frame: 1,
        }
    }
}

impl eframe::App for SocialSimulationApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        // Put configuration panel on the left
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");
            ui.add(egui::Slider::new(&mut self.person_radius, 1.0..=10.0).text("Radius"));
            if ui.button("Increment").clicked() {
                self.person_radius += 1.0;
            }

            ui.separator();

            if ui.button("Pause").clicked() {
                if self.steps_per_frame > 0 {
                    self.steps_per_frame = 0;
                } else {
                    self.steps_per_frame = 1;
                }
            }
            ui.add(egui::Slider::new(&mut self.steps_per_frame, 0..=200).text("Steps per frame"));

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Social Simulation");

            let (response, painter) = ui.allocate_painter(
                Vec2::new(800.0, 600.0),
                Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            painter.rect_stroke(
                response.rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 100)),
            );

            // Draw circles on a painter
            for p in self.person_positions.iter() {
                 let p = Vec2::new(p.0, p.1);
                painter.circle_filled(
                    response.rect.left_top() + p,
                    self.person_radius,
                    egui::Color32::from_rgb(100, 200, 100),
                );
            }

            for _ in 0..self.steps_per_frame {
                for i in 0..200 as usize {
                    self.person_positions[i].0 += self.person_speeds[i].0;
                    self.person_positions[i].1 += self.person_speeds[i].1;

                    if self.person_positions[i].0 > 800 as f32 {
                        self.person_positions[i].0 = 0.0;
                    } else if self.person_positions[i].0 < 0.0 {
                        self.person_positions[i].0 = 800 as f32;
                    }
                    if self.person_positions[i].1 > 600 as f32 {
                        self.person_positions[i].1 = 0.0;
                    } else if self.person_positions[i].1 < 0.0 {
                        self.person_positions[i].1 = 600 as f32;
                    }
                }
            }

        });

        // Quit app if Escape is pressed
        if ctx.input(|i| i.key_down(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Force repaint
        ctx.request_repaint();
    }
}

pub(crate) fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}