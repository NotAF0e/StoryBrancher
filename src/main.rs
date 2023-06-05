use eframe::egui;
use egui::{Pos2, Vec2};
use std::{fs, fs::File, io::Read, process::exit};

#[derive(PartialEq)]
enum AppState {
    MainMenu,
    Playing,
    Creating,
}

fn main() {
    pub struct App {
        state: AppState,
    }
    impl Default for App {
        fn default() -> Self {
            Self {
                state: AppState::MainMenu,
            }
        }
    }

    let paths = fs::read_dir("./.stories").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        println!("{:?}:", path.display());

        for f in fs::read_dir(path).unwrap() {
            let mut f = File::open(f.unwrap().path()).unwrap();
            let mut f_contents = String::new();
            f.read_to_string(&mut f_contents).unwrap();

            println!("{:?}\n", f_contents);
        }
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                if self.state == AppState::MainMenu {
                    ui.vertical(|ui| {
                        if ui.button("Play").clicked() {
                            self.state = AppState::Playing;
                        } else if ui.button("Create Pathed Story").clicked() {
                            self.state = AppState::Creating;
                        }
                    });
                } else if self.state == AppState::Playing {
                    egui::Window::new("")
                        .fixed_pos(Pos2 {
                            x: (frame.info().window_info.size.x / 4.0),
                            y: (frame.info().window_info.size.y / 19.0),
                        })
                        .fixed_size([
                            frame.info().window_info.size.x / 2.0,
                            frame.info().window_info.size.y / 1.2,
                        ])
                        .show(ctx, |ui| {
                            egui::ScrollArea::new([false, true])
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                )
                                .show(ui, |ui| {
                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new("Hello, world! ".repeat(1000))
                                            .size(20.0),
                                    );
                                    ui.separator();
                                });
                        });
                } else {
                    exit(0);
                }

                ctx.request_repaint();
            });
        }
    }

    let options = eframe::NativeOptions {
        maximized: true,
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        follow_system_theme: true,
        centered: true,
        min_window_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Story Pather",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
    .expect("OUCH");
}
