use eframe::{egui, IconData};
use egui::{Align, Pos2, Vec2};
use std::{fs, fs::File, io, io::prelude::*, io::BufReader, process::exit};

#[derive(Debug)]
struct Story {
    name: String,
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    name: String,
    branches: Vec<Branch>,
    content: String,
}

#[derive(Debug, Clone)]
struct Branch {
    id: Option<usize>,
    name: Option<String>,
}

#[derive(PartialEq)]
enum AppState {
    MainMenu,
    Playing,
    Tips,
}
pub struct App {
    stories: Vec<Story>,

    current_story: usize,
    current_node: usize,
    state: AppState,

    text_size: f32,
}
impl Default for App {
    fn default() -> Self {
        Self {
            stories: vec![],

            current_story: 0,
            current_node: 0,
            state: AppState::MainMenu,

            text_size: 15.0,
        }
    }
}

fn load_stories() -> io::Result<Vec<Story>> {
    let paths = fs::read_dir("./.stories")?;

    let mut stories: Vec<Story> = vec![];

    for path in paths {
        if let Ok(path) = path {
            let mut story = Story {
                name: path.file_name().to_string_lossy().to_string(),
                nodes: vec![],
            };
            let story_path = path.path();

            for f in fs::read_dir(&story_path).unwrap() {
                if let Ok(dir_entry) = f {
                    let node_path = File::open(dir_entry.path()).unwrap();
                    let reader = BufReader::new(node_path);
                    let mut lines = reader.lines().map(|result| result.unwrap());

                    let branches: Vec<Branch> = {
                        lines
                            .nth(0)
                            .unwrap()
                            .split(", ")
                            .clone()
                            .map(|x| Branch {
                                id: x.parse::<usize>().ok(),
                                name: None,
                            })
                            .zip(lines.nth(0).unwrap().split(", "))
                            .map(|(mut b, x)| {
                                b.name = Some(x.to_string());
                                Branch {
                                    id: b.id,
                                    name: b.name,
                                }
                            })
                            .collect()
                    };

                    let mut content: String = Default::default();
                    let mut past_node_info = false;
                    for line in lines {
                        if past_node_info {
                            content.push_str(&("\n".to_string() + &line));
                        }
                        if line == "+++" {
                            past_node_info = true;
                        }
                    }
                    story.nodes.push(Node {
                        name: dir_entry.file_name().to_string_lossy().to_string(),
                        branches,
                        content,
                    });
                }
            }
            stories.push(story);
        }
    }
    Ok(stories)
}

fn main() {
    let stories = match load_stories() {
        Ok(stories) => stories,
        Err(err) => {
            eprintln!("Failed to load stories: {}", err);
            exit(1);
        }
    };

    println!("{:#?}", stories);
    let mut app = App::default();
    app.stories = stories;

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                if self.state == AppState::MainMenu {
                    ui.vertical(|ui| {
                        if ui.button("Play").clicked() {
                            self.state = AppState::Playing;
                        } //else if ui.button("Create Pathed Story").clicked() {
                          //  self.state = AppState::Tips;
                          //}
                    });
                } else if self.state == AppState::Playing {
                    ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                        egui::TopBottomPanel::bottom("Settings").show(ctx, |ui| {
                            egui::collapsing_header::CollapsingHeader::new("Text size").show(
                                ui,
                                |ui| {
                                    ui.add(
                                        egui::DragValue::new(&mut self.text_size)
                                            .clamp_range(5.0..=40.0),
                                    );
                                },
                            );
                        });
                    });

                    egui::Window::new("")
                        .fixed_pos(Pos2 {
                            x: (frame.info().window_info.size.x / 4.0),
                            y: (frame.info().window_info.size.y / 20.0),
                        })
                        .fixed_size([
                            frame.info().window_info.size.x / 2.0,
                            frame.info().window_info.size.y / 1.2,
                        ])
                        .collapsible(false)
                        .show(ctx, |ui| {
                            egui::ScrollArea::new([false, true])
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                )
                                .show(ui, |ui| {
                                    let current_node = &mut self.stories[self.current_story].nodes
                                        [self.current_node];

                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new(current_node.content.clone())
                                            .size(self.text_size),
                                    );

                                    for branch in current_node.branches.clone() {
                                        if let (Some(branch_id), Some(branch_name)) =
                                            (branch.id, branch.name)
                                        {
                                            ui.separator();

                                            if ui.button(branch_name).clicked() {
                                                self.current_node = branch_id;
                                            }
                                        }
                                    }
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
        min_window_size: Some(Vec2::new(800.0, 750.0)),
        icon_data: Some(
            IconData::try_from_png_bytes(include_bytes!("../assets/icon.png")).unwrap(),
        ),
        ..Default::default()
    };

    eframe::run_native("Story Brancher", options, Box::new(|_cc| Box::new(app))).expect("OUCH");
}
