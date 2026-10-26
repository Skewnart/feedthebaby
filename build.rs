fn main() {
    // Scan the project for Tailwind classes and generate the stylesheet.
    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .unwrap();

    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("lucide")
        .stage()
        .unwrap();
}