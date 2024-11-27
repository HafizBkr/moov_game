use eframe::egui::{CentralPanel, Context, Key};
use std::collections::HashSet;

struct App {
    position: (i32, i32),
    deplacement: i32,
    score: i32,
    visited_positions: HashSet<(i32, i32)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            position: (0, 0),
            deplacement: 0,
            score: 0,
            visited_positions: HashSet::new(),
        }
    }
}

impl App {
    fn update_position(&mut self, direction: &str) {
        let movement_cost = match direction {
            "haut" => (0, -1, 25),
            "bas" => (0, 1, 90),
            "gauche" => (-1, 0, 50),
            "droite" => (1, 0, 70),
            _ => (0, 0, 0),
        };

        self.position = (
            self.position.0 + movement_cost.0,
            self.position.1 + movement_cost.1,
        );

        self.deplacement += 1;

        if self.visited_positions.contains(&self.position) {
            self.score -= 25; // Décrémenter si revisité
        } else {
            self.visited_positions.insert(self.position);
            self.score += movement_cost.2 / self.deplacement; // Bonus basé sur déplacements
        }

        // Réinitialisation du score si retour au départ
        if self.position == (0, 0) {
            self.score = 0;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Déplacement et Calcul de Score");

            ui.label(format!(
                "Position actuelle : ({}, {})",
                self.position.0, self.position.1
            ));
            ui.label(format!("Déplacements : {}", self.deplacement));
            ui.label(format!("Score : {}", self.score));

            ui.horizontal(|ui| {
                if ui.button("⬆️ Haut").clicked() {
                    self.update_position("haut");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("⬅️ Gauche").clicked() {
                    self.update_position("gauche");
                }
                if ui.button("➡️ Droite").clicked() {
                    self.update_position("droite");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("⬇️ Bas").clicked() {
                    self.update_position("bas");
                }
            });

            // Instructions
            ui.add_space(10.0);
            ui.label("Utilisez les boutons pour déplacer le personnage !");
        });

        // Rafraîchit l'interface pour détecter les actions clavier
        if ctx.input(|i| i.key_pressed(Key::W)) {
            self.update_position("haut");
        }
        if ctx.input(|i| i.key_pressed(Key::A)) {
            self.update_position("gauche");
        }
        if ctx.input(|i| i.key_pressed(Key::S)) {
            self.update_position("bas");
        }
        if ctx.input(|i| i.key_pressed(Key::D)) {
            self.update_position("droite");
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Algorithme de Déplacement",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
