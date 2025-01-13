use render::Render;

mod render;

fn main() {
    let render = Render::new();
    render.render(640, 480, 4, 2);
}
