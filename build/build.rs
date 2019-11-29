mod out;
mod resources;
mod glade;

fn main() {

    out::output_dir();
    resources::generate_resources();
    glade::fix_resource_paths();

    #[cfg(target_os = "windows")]
    {
        #[path = "windows.rs"]
        mod windows;
        windows::generate_rc();
        windows::compile_rc();
    }

}
