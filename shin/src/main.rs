mod asset;
// mod camera;
mod adv;
mod interpolator;
mod layer;
mod render;
mod update;
mod vm;

fn main() {
    // old_main()
    pollster::block_on(render::run());
}
