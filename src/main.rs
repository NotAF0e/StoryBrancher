use eframe::egui;
use egui::{Pos2, Vec2};
use std::{fs, fs::File, io, io::prelude::*, io::BufReader, process::exit};

#[derive(PartialEq)]
enum AppState {
    MainMenu,
    Playing,
    Creating,
}
pub struct App {
    stories: Vec<Story>,

    current_story: usize,
    current_node: usize,
    state: AppState,
}
impl Default for App {
    fn default() -> Self {
        Self {
            stories: vec![],

            current_story: 0,
            current_node: 0,
            state: AppState::MainMenu,
        }
    }
}

#[derive(Debug)]
struct Story {
    name: String,
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    name: String,
    branches: Vec<Option<usize>>,
    content: String,
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

                    let branches: Vec<Option<usize>> = lines
                        .nth(0)
                        .unwrap()
                        .split(", ")
                        .map(|x| {
                            if x.parse::<usize>().is_ok() {
                                Some(x.parse::<usize>().unwrap())
                            } else {
                                None
                            }
                        })
                        .collect();

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
                                    let current_node = &mut self.stories[self.current_story].nodes
                                        [self.current_node];

                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new(current_node.content.clone())
                                            .size(20.0),
                                    );

                                    for branch in current_node.branches.clone() {
                                        if let Some(branch) = branch {
                                            ui.separator();
                                            if ui
                                                .button(
                                                    &self.stories[self.current_story].nodes[branch]
                                                        .name,
                                                )
                                                .clicked()
                                            {
                                                self.current_node = branch;
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
        min_window_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native("Story Pather", options, Box::new(|_cc| Box::new(app))).expect("OUCH");
}
