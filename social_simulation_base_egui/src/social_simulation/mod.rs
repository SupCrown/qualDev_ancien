use egui::{Sense, Vec2};
use rand::Rng;
use person::illness_state::IllnessState;
use person::Person;
use person::position::Position;
use person::vec2d::Vec2D;
use world::World;

pub mod person;
pub mod world;

pub struct SocialSimulation{
    pub(crate) population:Vec<Person>,
    pub world: World,
    pub person_radius : f32,
    pub steps_per_frame : u32
}

impl SocialSimulation{
    pub fn new(world: World, _cc: &eframe::CreationContext<'_>)-> Self{
        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        let mut popu:Vec<Person> = vec![];

        for _ in 0..200 {
           let position:Position = Position::new(rng1.gen_range(0..world.height) as f64, rng2.gen_range(0..world.width) as f64);
            let velocity:Vec2D = Vec2D {
                vx:rng1.gen_range( - 1.0..=1.0) as f64,
                vy:rng2.gen_range( - 1.0..=1.0) as f64,
            };
            let person = Person::new(world, position, velocity, IllnessState::Healthy);
            popu.push(person);
        }
        SocialSimulation{
            population:popu,
            world,
            person_radius:0.1,
            steps_per_frame:1,
        }
    }

    pub fn step(&mut self){
        for person in &mut self.population {
            person.step();
        }
    }
}


impl eframe::App for SocialSimulation {
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
                crate::app::powered_by_egui_and_eframe(ui);
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
            for person in self.population.iter() {
                let p = Vec2::new(person.position.x as f32, person.position.y as f32);
                painter.circle_filled(
                    response.rect.left_top() + p,
                    self.person_radius,
                    egui::Color32::from_rgb(100, 200, 100),
                );
            }

            for _ in 0..self.steps_per_frame {
                self.step();
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