use std::{collections::VecDeque, time::Duration};

use eframe::{egui, epaint::ahash::HashSet};
use egui::{Color32, Sense, Stroke};
use nom::{combinator::all_consuming, Finish};
use parse::{Direction, Instruction};

use crate::parse::GridPos;

mod parse;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|_cc| Box::new(MyApp::new())),
        )
        .await
        .expect("failed to start eframe");
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 - Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    instructions: VecDeque<Instruction>,
    head: GridPos,
    tail: GridPos,
    tail_visited: HashSet<GridPos>,
}

impl MyApp {
    fn new() -> Self {
        let instructions = include_str!("sample-input.txt")
            .lines()
            .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
            .collect();

        Self {
            instructions,
            head: GridPos { x: 0, y: 0 },
            tail: GridPos { x: 0, y: 0 },
            tail_visited: Default::default(),
        }
    }

    fn update_state(&mut self) {
        let instruction = match self.instructions.front_mut() {
            Some(instruction) => instruction,
            None => return,
        };
        self.head += instruction.dir.delta();

        let diff = self.head - self.tail;

        let (dx, dy) = match (diff.x, diff.y) {
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            _ => panic!("unhandled case: tail - head = {diff:?}"),
        };

        self.tail.x += dx;
        self.tail.y += dy;
        self.tail_visited.insert(self.tail);

        instruction.dist -= 1;
        if instruction.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_state();

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.label(format!("{} instructions left", self.instructions.len()));
            ui.label(format!("{} places visited", self.tail_visited.len()));

            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                for ins in &self.instructions {
                    let arrow = match ins.dir {
                        Direction::Up => "⬆",
                        Direction::Down => "⬇",
                        Direction::Right => "➡",
                        Direction::Left => "⬅",
                    };
                    ui.label(arrow.repeat(ins.dist as _));
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            const CANVAS_WIDTH: f32 = 900.0;
            const CANVAS_HEIGHT: f32 = 700.0;
            const SIDE: f32 = 5.0;

            let painter_size = egui::vec2(CANVAS_WIDTH, CANVAS_HEIGHT);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            let to_panel_pos = |pos: GridPos| {
                (egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE) + center).to_pos2()
            };

            let half_width = { CANVAS_WIDTH / SIDE }.floor() as i32;
            let half_height = { CANVAS_HEIGHT / SIDE }.floor() as i32;

            for x in -half_width..half_width {
                for y in -half_height..half_height {
                    let dot = GridPos { x, y };
                    if !self.tail_visited.contains(&dot) {
                        continue;
                    }
                    let color = Color32::DARK_RED;

                    let dot_pos = to_panel_pos(dot);
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }

            let head_pos = to_panel_pos(self.head);
            painter.circle_stroke(head_pos, 2.0, Stroke::new(2.0, Color32::GREEN));

            let tail_pos = to_panel_pos(self.tail);
            painter.circle_stroke(tail_pos, 2.0, Stroke::new(2.0, Color32::YELLOW));

            painter.arrow(
                tail_pos,
                head_pos - tail_pos,
                Stroke::new(2.0, Color32::YELLOW),
            )
        });

        ctx.request_repaint_after(Duration::from_millis(25));
    }
}
