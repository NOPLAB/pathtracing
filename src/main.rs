use render::Render;

mod render;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let render = Render::new();
    render.render(640, 480, 10, 5, 16);

    let elapsed = now.elapsed();
    println!("Total Δt = {:.4?}", elapsed);
}
