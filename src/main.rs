use render::Render;

mod render;

fn main() {
    let render = Render::new();
    println!("{} CPUs detected", num_cpus::get());
    render.render(640, 480, 10, 5, num_cpus::get() as u32);
}
