use crate::social_simulation::person::illness_state::IllnessState;
use crate::social_simulation::SocialSimulation;
use eframe::egui;
use eframe::epaint::Color32;
use egui::{Sense, Vec2};
use egui_plot::{Line, Plot, PlotPoints};

pub struct SocialSimulationApp {
    social_simulation: SocialSimulation,
    person_radius: f32,
    steps_per_frame: u32,
    show_stats: bool,
    last_update_time: f64,
}

impl SocialSimulationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        SocialSimulationApp {
            social_simulation: SocialSimulation::new().unwrap(),
            person_radius: 3.0,
            steps_per_frame: 1,
            show_stats: false,
            last_update_time: 0.0,
        }
    }
}

impl eframe::App for SocialSimulationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Limiter les mises à jour à 60 FPS
        let current_time = ctx.input(|i| i.time);
        if current_time - self.last_update_time >= 1.0 / 60.0 {
            self.last_update_time = current_time;

            for _ in 0..self.steps_per_frame {
                self.social_simulation.step();
            }
        }

        // Panneaux de l'interface utilisateur
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if !cfg!(target_arch = "wasm32") {
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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");
            ui.add(egui::Slider::new(&mut self.person_radius, 1.0..=10.0).text("Radius"));
            ui.add(egui::Slider::new(&mut self.steps_per_frame, 0..=200).text("Steps per frame"));
            ui.add(egui::Slider::new(&mut self.social_simulation.infection_percent, 0.0..=1.0).text("Infection chances"));
            ui.add(egui::Slider::new(&mut self.social_simulation.mortality, 0.0..=1.0).text("Mortality"));
            ui.add(egui::Slider::new(&mut self.social_simulation.recovery_chances, 0.0..=1.0).text("Recovery chances"));

            if ui.button("Pause").clicked() {
                self.steps_per_frame = if self.steps_per_frame > 0 { 0 } else { 1 };
            }

            if ui.button("Infect Someone").clicked() {
                self.social_simulation.infect_someone();
            }

            ui.separator();

            if ui.button("Show Stats").clicked() {
                self.show_stats = !self.show_stats;
            }

            if self.show_stats {
                ui.heading("Info");
                ui.label(format!("Population: {}", self.social_simulation.total_population));
                ui.label(format!("Healthy: {}", self.social_simulation.total_healthy));
                ui.label(format!("Infected: {}", self.social_simulation.total_infected));
                ui.label(format!("Dead: {}", self.social_simulation.total_dead));
                ui.label(format!("Recovered: {}", self.social_simulation.total_recovered));
                ui.label(format!("Masks: {}", self.social_simulation.total_masks));
                ui.label(format!("Vaccinated: {}", self.social_simulation.total_vaccinated));
                ui.label(format!("Mortality: {:.2}", self.social_simulation.mortality));
            }
        });

        // Fenêtre principale pour la simulation et le graphique
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Social Simulation");

            // Utiliser une disposition horizontale
            ui.horizontal(|ui| {
                // Partie gauche : Simulation
                ui.vertical(|ui| {
                    // Rendre les cercles pour chaque personne
                    let (response, painter) = ui.allocate_painter(
                        Vec2::new(
                            self.social_simulation.world.width as f32,
                            self.social_simulation.world.height as f32,
                        ),
                        Sense::hover(),
                    );

                    painter.rect_stroke(
                        response.rect,
                        0.0,
                        egui::Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
                    );

                    for p in self.social_simulation.population.iter() {
                        let pos = Vec2::new(p.position.x as f32, p.position.y as f32);
                        let color = match p.state {
                            IllnessState::Infected => Color32::from_rgb(200, 0, 0),
                            IllnessState::Healthy => Color32::from_rgb(0, 200, 0),
                            IllnessState::Dead => Color32::from_rgb(255, 255, 255),
                            IllnessState::Recovered => Color32::from_rgb(0, 0, 200),
                        };
                        painter.circle_filled(response.rect.left_top() + pos, self.person_radius, color);
                    }
                });

                // Partie droite : Graphique
                ui.vertical(|ui| {
                    let plot = Plot::new("State History")
                        .legend(egui_plot::Legend::default())
                        .view_aspect(2.0)
                        .height(400.0) // Ajuster la hauteur du graphique
                        .allow_zoom(true)
                        .allow_drag(true);

                    plot.show(ui, |plot_ui| {
                        plot_ui.line(Line::new(PlotPoints::from_iter(
                            self.social_simulation.history_steps.iter().zip(self.social_simulation.history_healthy.iter()).map(|(&x, &y)| [x as f64, y as f64]),
                        )).name("Healthy").color(Color32::from_rgb(0, 255, 0)));

                        plot_ui.line(Line::new(PlotPoints::from_iter(
                            self.social_simulation.history_steps.iter().zip(self.social_simulation.history_infected.iter()).map(|(&x, &y)| [x as f64, y as f64]),
                        )).name("Infected").color(Color32::from_rgb(255, 0, 0)));

                        plot_ui.line(Line::new(PlotPoints::from_iter(
                            self.social_simulation.history_steps.iter().zip(self.social_simulation.history_dead.iter()).map(|(&x, &y)| [x as f64, y as f64]),
                        )).name("Dead").color(Color32::from_rgb(255, 255, 255)));

                        plot_ui.line(Line::new(PlotPoints::from_iter(
                            self.social_simulation.history_steps.iter().zip(self.social_simulation.history_recovered.iter()).map(|(&x, &y)| [x as f64, y as f64]),
                        )).name("Recovered").color(Color32::from_rgb(0, 0, 255)));
                    });
                });
            });
        });

        ctx.request_repaint();
    }
}