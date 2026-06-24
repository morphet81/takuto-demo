#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn hero_heading_is_bonjour_monde() {
        let app = fs::read_to_string("src/App.jsx").expect("read src/App.jsx");
        assert!(
            app.contains("<h1>Bonjour Monde</h1>"),
            "hero heading should be Bonjour Monde"
        );
    }
}
