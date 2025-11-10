use chrono::{Local, Utc};
use chrono_tz::Tz;
use eframe::egui;

#[derive(Debug, Clone)]
pub struct WorldTime {
    name: String,
    time: String,        // HH:MM format
    diff_hours: i32,     // hours difference from home time
    is_home: bool,       // true if this is your home location
    timezone_id: String, // like "Europe/Berlin" or "America/Chicago"
}

impl WorldTime {
    fn new(name: &str, timezone_id: &str, is_home: bool, home_offset: i32) -> Self {
        // Calculate actual time for this timezone
        let current_time = if let Ok(tz) = timezone_id.parse::<Tz>() {
            let utc_now = Utc::now();
            utc_now.with_timezone(&tz).format("%H:%M").to_string()
        } else {
            // Fallback to local time if timezone parsing fails
            Local::now().format("%H:%M").to_string()
        };

        WorldTime {
            name: name.to_string(),
            time: current_time,
            diff_hours: home_offset,
            is_home,
            timezone_id: timezone_id.to_string(),
        }
    }

    fn update_time(&mut self) {
        // Update the time for this timezone
        let current_time = if let Ok(tz) = self.timezone_id.parse::<Tz>() {
            let utc_now = Utc::now();
            utc_now.with_timezone(&tz).format("%H:%M").to_string()
        } else {
            Local::now().format("%H:%M").to_string()
        };
        self.time = current_time;
    }
}

struct WorldTimeApp {
    cities: Vec<WorldTime>,
}

impl Default for WorldTimeApp {
    fn default() -> Self {
        let austin = WorldTime::new("Austin", "America/Chicago", true, 0);
        let nyc = WorldTime::new("NYC", "America/New_York", false, 1);
        let london = WorldTime::new("London", "Europe/London", false, 6);
        let berlin = WorldTime::new("Berlin", "Europe/Berlin", false, 7);
        let bucharest = WorldTime::new("Bucharest", "Europe/Bucharest", false, 8);

        Self {
            cities: vec![austin, nyc, london, berlin, bucharest],
        }
    }
}

impl eframe::App for WorldTimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update all city times every frame
        for city in &mut self.cities {
            city.update_time();
        }

        // Request repaint to keep updating times
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌍 World Time Display");

            ui.separator();

            // Display all cities in a responsive grid (all visible at once)
            egui::Grid::new("cities_grid")
                .spacing([15.0, 10.0])
                .show(ui, |ui| {
                    for (index, city) in self.cities.iter().enumerate() {
                        // City card with enhanced styling
                        let frame_color = if city.is_home {
                            egui::Color32::from_rgb(59, 130, 246) // Blue border for home
                        } else {
                            ui.style().visuals.widgets.noninteractive.bg_fill
                        };

                        egui::Frame::group(ui.style())
                            .stroke(egui::Stroke::new(2.0, frame_color))
                            .fill(if city.is_home {
                                egui::Color32::from_rgb(240, 249, 255) // Light blue background for home
                            } else {
                                egui::Color32::from_rgb(249, 250, 251) // Light gray for others
                            })
                            .rounding(8.0)
                            .show(ui, |ui| {
                                ui.set_min_width(180.0);
                                ui.vertical_centered(|ui| {
                                    ui.add_space(8.0);

                                    // City name - bigger and highlighted for home
                                    if city.is_home {
                                        ui.label(
                                            egui::RichText::new(format!("🏠 {}", &city.name))
                                                .size(20.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(59, 130, 246)),
                                        );
                                    } else {
                                        ui.heading(&city.name);
                                    }

                                    ui.add_space(6.0);

                                    // Current time - BIGGER and more prominent
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("🕐").size(24.0));
                                        ui.label(
                                            egui::RichText::new(&city.time)
                                                .size(32.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(17, 24, 39)),
                                        );
                                    });

                                    ui.add_space(6.0);

                                    // Time difference with better styling
                                    let diff_color = if city.diff_hours >= 0 {
                                        egui::Color32::from_rgb(34, 197, 94) // Green-500
                                    } else {
                                        egui::Color32::from_rgb(239, 68, 68) // Red-500
                                    };

                                    ui.colored_label(
                                        diff_color,
                                        egui::RichText::new(format!("Δ {} hours", city.diff_hours))
                                            .strong(),
                                    );

                                    // Timezone
                                    ui.horizontal(|ui| {
                                        ui.label("🌍");
                                        ui.label(
                                            egui::RichText::new(&city.timezone_id)
                                                .color(egui::Color32::from_rgb(107, 114, 128)),
                                        );
                                    });

                                    ui.add_space(8.0);
                                });
                            });

                        // New row every 3 cities for responsive layout
                        if (index + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([650.0, 450.0]) // More reasonable window size
            .with_title("World Time Display")
            .with_resizable(false), // Disable resizing (which includes maximizing)
        ..Default::default()
    };

    eframe::run_native(
        "World Time Display",
        options,
        Box::new(|_cc| Ok(Box::new(WorldTimeApp::default()))),
    )
}
