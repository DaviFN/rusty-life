mod game;
use egui::color_picker::Alpha;
use game::Game;
use crate::app::game::field::CellState;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    cell_size: usize,
    cell_border_size: usize,
    living_cell_color: egui::Color32,
    dead_cell_color: egui::Color32,
    border_color: egui::Color32,

    n_generations_to_advance: usize,

    new_game_width: usize,
    new_game_height: usize,

    probability_living_cell: f64,

    consider_extremes_adjacent: bool,

    #[serde(skip)] // This how you opt-out of serialization of a field
    debug_message: String,

    game: Game,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let default_probability_living_cell: f64 = 30.0;
        let default_game_width = 20;
        let default_game_height = 20;
        let mut game: Game = Game::new(default_game_width, default_game_height);
        game.randomize(default_probability_living_cell);
        Self {
            cell_size: 15,
            cell_border_size: 1,
            living_cell_color: egui::Color32::GREEN,
            dead_cell_color: egui::Color32::GRAY,
            border_color: egui::Color32::BLACK,
            n_generations_to_advance: 100,
            new_game_width: default_game_width,
            new_game_height: default_game_height,
            probability_living_cell: default_probability_living_cell,
            consider_extremes_adjacent: true,
            debug_message: String::from("<NO DEBUG MESSAGE>"),
            game
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn draw_board(&mut self, ui: &mut egui::Ui, response: &egui::Response, painter: &egui::Painter) {
        let cell_width = self.cell_size;
        let cell_height = self.cell_size;
        let cell_border_size = self.cell_border_size;
        for i in 0..self.game.get_field().get_width() {
            for j in 0..self.game.get_field().get_height() {
                let cell_state: game::field::CellState = self.game.get_field().get_cell_state(i, j);
                let cell_color: egui::Color32 = if cell_state == game::field::CellState::Alive {self.living_cell_color} else {self.dead_cell_color};
                draw_rectangle(ui, &response, &painter, cell_width * i, cell_height * j, cell_width, cell_height, self.border_color);
                draw_rectangle(ui, &response, &painter, cell_width * i + cell_border_size, cell_height * j + cell_border_size, cell_width - 2 * cell_border_size, cell_height - 2 * cell_border_size, cell_color);
            }
        }
    }

    fn on_mouse_press_on_game_window(&mut self, pos : egui::Pos2, was_right_click: bool) {
        let x_pressed = pos.x as usize;
        let y_pressed = pos.y as usize;
        let x_within_square_representation = x_pressed % self.cell_size;
        let y_within_square_representation = y_pressed % self.cell_size;
        let x_was_within_square : bool = self.cell_border_size < x_within_square_representation && x_within_square_representation + self.cell_border_size < self.cell_size;
        let y_was_within_square : bool = self.cell_border_size < y_within_square_representation && y_within_square_representation + self.cell_border_size < self.cell_size;
        if x_was_within_square && y_was_within_square {
            let x_clicked_cell = x_pressed / self.cell_size;
            let y_clicked_cell = y_pressed / self.cell_size;
            self.cell_clicked(x_clicked_cell, y_clicked_cell, was_right_click);
        }
        //self.debug_message = "Mouse button at: ".to_owned() + &x_pressed.to_string() + "-" + &y_pressed.to_string();
    }

    fn cell_clicked(&mut self, x: usize, y: usize, was_right_click: bool) {
        let new_cell_state: CellState = if was_right_click {CellState::Dead} else {CellState::Alive};
        self.game.get_field().set_cell_state(x, y, new_cell_state);
        //self.debug_message = "Clicked cell: ".to_owned() + &x.to_string() + "-" + &y.to_string();
    }

    fn show_color_controls_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Color controls")
            .resizable(true)
            .collapsible(true)
            .default_open(false)
            .default_pos(egui::pos2(200.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Living cell color:");
                        egui::widgets::color_picker::color_picker_color32(ui, &mut self.living_cell_color, Alpha::Opaque);
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("Dead cell color:");
                        egui::widgets::color_picker::color_picker_color32(ui, &mut self.dead_cell_color, Alpha::Opaque);
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("Border color:");
                        egui::widgets::color_picker::color_picker_color32(ui, &mut self.border_color, Alpha::Opaque);
                    });
                });
            });
    }
}

fn draw_rectangle(ui :&mut egui::Ui, response: &egui::Response, painter: &egui::Painter, starting_x: usize, starting_y: usize, width: usize, height: usize, color : egui::Color32)
{
    let absolute_position = egui::Pos2 {
        x: response.rect.min.x,
        y: response.rect.min.y
    };

    let max_x = response.rect.right();
    let max_y = response.rect.bottom();

    let x: f32 = if width as f32 <= max_x {width as f32} else {max_x};
    let y: f32 = if height as f32 <= max_y {height as f32} else {max_y};

    let my_rect = egui::Rect::from_min_size(
        egui::Pos2::new(starting_x as f32 + absolute_position.x, starting_y as f32 + absolute_position.y),
        egui::vec2(x, y)
    );
    
    painter.rect_filled(my_rect, 0.0, color);
}

fn check_pressed_button_within_game_window(response: &egui::Response, ui: &egui::Ui, pointer_button: egui::PointerButton) -> Option<egui::Pos2> {
    if ui.input(|input| input.pointer.button_pressed(pointer_button)) {
        if let Some(pos) = ui.input(|input| input.pointer.press_origin()) {
            if !response.rect.contains(pos) {
                return None;
            }
            let absolute_position = egui::Pos2 {
                x: response.rect.min.x,
                y: response.rect.min.y
            };
            let position_within_window: egui::Pos2 = egui::Pos2::new(pos.x - absolute_position.x, pos.y - absolute_position.y); 
            return Some(position_within_window)
        }
    }
    None
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        self.show_color_controls_window(ctx);

        egui::Window::new("Game window")
            .resizable(true)
            .collapsible(true)
            .default_pos(egui::pos2(400.0, 0.0))
            .show(ctx, |ui| {
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), ui.available_height()), // Set width and height of the drawing area
                    egui::Sense::hover(),
                );

                let left_pressed_position: Option<egui::Pos2> = check_pressed_button_within_game_window(&response, &ui, egui::PointerButton::Primary);
                if let Some(left_pressed_position) = left_pressed_position {
                    self.on_mouse_press_on_game_window(left_pressed_position, false /*was_right_click*/);
                }

                let right_pressed_position: Option<egui::Pos2> = check_pressed_button_within_game_window(&response, &ui, egui::PointerButton::Secondary);
                if let Some(right_pressed_position) = right_pressed_position {
                    self.on_mouse_press_on_game_window(right_pressed_position, true /*was_right_click*/);
                }

                self.draw_board(ui, &response, &painter);
            });
            

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Rusty life");

            //ui.separator();
            //ui.label("Debug messages");
            //ui.label(self.debug_message.clone());

            ui.separator();
            ui.label("Game display options");

            let game_dimensions_text = String::from("Currently simulating a ") + &self.game.get_field().get_width().to_string() + &String::from("x") + &self.game.get_field().get_width().to_string() + &String::from(" game");
            ui.label(game_dimensions_text);

            ui.add(egui::Slider::new(&mut self.cell_size, 1 as usize..=100 as usize).text("Cell size"));
            ui.add(egui::Slider::new(&mut self.cell_border_size, 1 as usize..=100 as usize).text("Cell border size"));
            if self.cell_border_size * 2 >= self.cell_size {
                self.cell_border_size = self.cell_size / 2;
                if self.cell_size % 2 == 0 {
                    self.cell_border_size = self.cell_border_size - 1;
                }
            }

            ui.separator();
            ui.label("Game control options");

            ui.add(egui::Slider::new(&mut self.new_game_width, 2 as usize..=1000 as usize).text("New game width"));
            ui.add(egui::Slider::new(&mut self.new_game_height, 2 as usize..=1000 as usize).text("New game height"));
            ui.add(egui::Slider::new(&mut self.probability_living_cell, 0.0..=100.0).text("Probability of living cell (percentage)"));
            let start_new_game_text = String::from("Start new ") + &self.new_game_width.to_string() + &String::from("x") + &self.new_game_height.to_string() + &String::from(" game");
            if ui.button(start_new_game_text).clicked() {
                self.game = Game::new(self.new_game_width, self.new_game_height);
                self.game.randomize(self.probability_living_cell);
            }

            if ui.button("Randomize cells").clicked() {
                self.game.randomize(self.probability_living_cell);
            }

            ui.label("Left click a cell to bring it to life");
            ui.label("Right click a cell to kill it");

            if ui.button("Kill'em all").clicked() {
                self.game.clear();
            }

            ui.separator();
            ui.label("Game simulation options");

            let current_generation_text = String::from("Current generation: ") + &self.game.get_generation().to_string();
            ui.label(current_generation_text);

            ui.checkbox(&mut self.consider_extremes_adjacent, "Consider cells at the extremes adjacent to one another").on_hover_text("Checking this will make it so that the cells at the extremes of the board will be considered neighbours to their opposite cells in any direction; this makes it possible for gliders to propel indefinitely across the board");

            if ui.button("Advance generation").clicked() {
                self.game.advance_generation(self.consider_extremes_adjacent);
            }

            ui.add(egui::Slider::new(&mut self.n_generations_to_advance, 2 as usize..=1000 as usize).text("Number of generations to advance"));

            let advance_n_generations_text = String::from("Advance ") + &self.n_generations_to_advance.to_string() + &String::from(" generations");
            if ui.button(advance_n_generations_text).clicked() {
                for i in 0..self.n_generations_to_advance {
                    self.game.advance_generation(self.consider_extremes_adjacent);
                }
            }

            ui.separator();

            ui.add(egui::Hyperlink::from_label_and_url("Feel free to take a look at the source code and/or contribute", "https://github.com/DaviFN/rusty-life").open_in_new_tab(true));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
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
