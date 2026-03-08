fn main() {
    //let config = slint_build::CompilerConfiguration::default();
    slint_build::compile("src/ui/index.slint").expect("Błąd kompilacji interfejsu (index.slint)");
}
