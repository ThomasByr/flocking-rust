extern crate find_folder;
extern crate fps_counter;
extern crate math_vector;
extern crate piston_window;

use flocking::boid::Boid;
use flocking::qtree::bounds::Point;
use flocking::qtree::QTree;

use fps_counter::*;
use math_vector::Vector;
use piston_window::*;

const N: usize = 100;
const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 600.0;
const CAPACITY: usize = 2;

fn main() {
    // Create boids
    let mut boids = Vec::new();
    for _i in 0..N {
        let boid = Boid::new(WIDTH, HEIGHT);
        boids.push(boid);
    }

    // Create qtree
    let mut qt = QTree::new(
        Vector::new(WIDTH / 2., HEIGHT / 2., 0.),
        CAPACITY,
        WIDTH / 2.0,
        HEIGHT / 2.0,
    );

    // Create fps counter
    let mut fps_counter = FPSCounter::new();
    let mut frames: usize;

    // Create Piston Window
    let mut window: PistonWindow = WindowSettings::new("Flocking", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .resizable(false)
        .vsync(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // Load Font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("RobotoMono-Thin.ttf"))
        .unwrap();

    // Main Loop
    window.set_position([100, 50]);
    window.set_max_fps(60);
    window.set_ups(60);
    while let Some(e) = window.next() {
        frames = fps_counter.tick(); // update fps counter

        // Create quadtree
        qt.clear();
        for &boid in boids.iter() {
            let point = Point {
                pos: boid.position.clone(),
                data: Some(boid),
            };
            qt.insert(&point);
        }

        // Update Boids
        for boid in boids.iter_mut() {
            let points = qt.query_boid(boid);
            let mut neighbors = Vec::new();
            for p in points.iter() {
                if let Some(b) = p.data {
                    if b.position != boid.position {
                        neighbors.push(b);
                    }
                }
            }
            boid.flock(&neighbors);
        }
        for boid in boids.iter_mut() {
            boid.update(WIDTH, HEIGHT);
        }

        // Draw
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10., 20.);
            clear([0.1; 4], g);

            // draw quadtree
            qt.draw(c, g, Some([0.2; 4]));

            // draw boids
            for boid in boids.iter_mut() {
                boid.draw(c, g, None);
            }

            // render fps on window as text
            text::Text::new_color([1.; 4], 9)
                .draw(
                    &mut format!("fps : {}", frames),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();
            glyphs.factory.encoder.flush(device); // update glyphs before rendering
        });
    }
}
