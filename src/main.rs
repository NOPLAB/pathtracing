use render::{ppm, Render, RenderConfig};

mod render;

fn main() {
    use std::time::Instant;

    let worker_count = std::env::var("WORKERS")
        .map(|s| s.parse().expect("Failed to parse env WORKERS"))
        .unwrap_or(16);

    let config = RenderConfig {
        tasks: worker_count,
        width: 640,
        height: 480,
        samples: 10,
        super_samples: 5,
    };

    let render = Render::new(config);

    let now = Instant::now();

    let image = render.render();

    let elapsed = now.elapsed();
    let score = (20.0 / elapsed.as_secs_f64()) * 1000.0;
    println!("Rendering Δt = {:.4?}", elapsed);

    println!("Saving image...");
    ppm::save_ppm("image.ppm", &image, config.width, config.height);
    let elapsed = now.elapsed();
    println!("Exporting Δt = {:.4?}", elapsed);

    let elapsed = now.elapsed();
    println!("Total Δt = {:.4?}", elapsed);
    println!("Score: {:.4} points", score);
}
