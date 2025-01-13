use render::Render;

mod render;

fn main() {
    let render = Render::new();
    render.render(1920, 1080, 4, 2);
}
