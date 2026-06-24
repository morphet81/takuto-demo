/// Greeting shown in the React app heading (`src/App.jsx`).
pub const GREETING: &str = "Bonjour Monde";

#[cfg(test)]
mod tests {
    use super::GREETING;

    #[test]
    fn greeting_matches_app_heading() {
        assert_eq!(GREETING, "Bonjour Monde");
    }
}
