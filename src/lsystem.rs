use std::collections::HashMap;

pub fn generate_l_system(
    axiom: &str,
    production_rules: HashMap<char, String>,
    n_iterations: u32,
) -> String {
    let l_system = LSystem::new(axiom, production_rules);
    l_system.generate(n_iterations)
}

struct LSystem {
    axiom: String,
    production_rules: HashMap<char, String>,
}

impl LSystem {
    fn new(axiom: &str, production_rules: HashMap<char, String>) -> Self {
        Self {
            axiom: axiom.to_string(),
            production_rules,
        }
    }

    fn generate(&self, n_iterations: u32) -> String {
        let mut s = self.axiom.clone();
        for _ in 0..n_iterations {
            s = self.apply_rules(&s);
        }
        s
    }

    fn apply_rules(&self, input: &str) -> String {
        input
            .chars()
            .map(|symbol| {
                if let Some(replacement) = self.production_rules.get(&symbol) {
                    replacement.clone()
                } else {
                    symbol.to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        // From wikipedia https://en.wikipedia.org/wiki/L-system
        let axiom = "A";
        let mut production_rules = HashMap::new();
        production_rules.insert('A', "AB".to_string());
        production_rules.insert('B', "A".to_string());

        let lsystem = LSystem::new(axiom, production_rules);

        assert_eq!(lsystem.generate(0), "A");
        assert_eq!(lsystem.generate(1), "AB");
        assert_eq!(lsystem.generate(2), "ABA");
        assert_eq!(lsystem.generate(3), "ABAAB");
        assert_eq!(lsystem.generate(4), "ABAABABA");
    }
}
