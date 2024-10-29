use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use rand::Rng;

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    basket_x: f64,
    basket_width: f64,
    items: Vec<(f64, f64)>, // Vector of (x, y) positions for falling items
    score: u32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Game {
        // Get the canvas and context
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        // Initialize game properties
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        Game {
            context,
            width,
            height,
            basket_x: width / 2.0 - 40.0,
            basket_width: 80.0,
            items: vec![],
            score: 0,
        }
    }

    pub fn update(&mut self) {
        // Clear screen with dark background
        self.context.set_fill_style(&JsValue::from_str("#333333")); // Dark background
        self.context.fill_rect(0.0, 0.0, self.width, self.height);

        // Spawn new item occasionally
        if rand::thread_rng().gen_range(0..20) == 0 {
            self.items.push((rand::thread_rng().gen_range(0.0..self.width), 0.0));
        }

        // Move items down
        for item in self.items.iter_mut() {
            item.1 += 2.0; // Move item down
        }

        // Check for caught items
        self.items.retain(|&(x, y)| {
            if y >= self.height - 20.0 && x >= self.basket_x && x <= self.basket_x + self.basket_width {
                self.score += 1;
                false // Remove caught item
            } else if y >= self.height {
                false // Remove missed item
            } else {
                true // Keep item
            }
        });

        // Draw basket with 3D shading
        let basket_gradient = self.context.create_linear_gradient(
            self.basket_x,
            self.height - 20.0,
            self.basket_x + self.basket_width,
            self.height,
        );
        basket_gradient.add_color_stop(0.0, "#005A9C").unwrap();
        basket_gradient.add_color_stop(0.5, "#00A2FF").unwrap();
        basket_gradient.add_color_stop(1.0, "#005A9C").unwrap();
        self.context.set_fill_style(&basket_gradient);
        self.context.fill_rect(self.basket_x, self.height - 20.0, self.basket_width, 20.0);

        // Draw items with 3D shading effect
        for &(x, y) in self.items.iter() {
            let gradient = self.context.create_radial_gradient(
                x - 2.0, y - 2.0, 2.0, // Light source near the top left of the sphere
                x, y, 10.0, // Radius of the gradient to cover the entire sphere
            ).unwrap();
            gradient.add_color_stop(0.0, "#FF6347").unwrap(); // Light red/orange for highlight
            gradient.add_color_stop(1.0, "#8B0000").unwrap(); // Dark red for shadow
            self.context.set_fill_style(&gradient);

            self.context.begin_path();
            self.context
                .arc(x, y, 10.0, 0.0, 2.0 * PI)
                .unwrap();
            self.context.fill();
        }

        // Draw score in a light color
        self.context.set_fill_style(&JsValue::from_str("#FFFFFF")); // White text
        self.context.set_font("20px sans-serif");
        self.context
            .fill_text(&format!("Score: {}", self.score), 10.0, 30.0)
            .unwrap();
    }

    pub fn move_basket(&mut self, direction: &str) {
        let move_amount = 20.0;
        match direction {
            "left" => {
                if self.basket_x > 0.0 {
                    self.basket_x -= move_amount;
                }
            }
            "right" => {
                if self.basket_x + self.basket_width < self.width {
                    self.basket_x += move_amount;
                }
            }
            _ => {}
        }
    }
}
