mod out;
mod resources;
mod glade;
mod windows;

fn main() {

    out::output_dir();
    resources::generate_resources();
    glade::fix_resource_paths();

    #[cfg(target_os = "windows")]
    {
        windows::generate_rc();
        windows::compile_rc();
    }

}
