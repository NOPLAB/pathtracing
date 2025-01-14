use render::Render;

mod render;

fn main() {
    let render = Render::new();
    let worker_count = std::env::var("WORKERS").map(|s| s.parse().expect("Failed to parse env WORKERS")).unwrap_or(16);
    render.render(640, 480, 10, 5, worker_count);
}
