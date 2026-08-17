fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let app = build_application(config)?;
    app.run()
}
