use std::collections::HashMap;

pub struct SymbolLookup {
    symbol_table: HashMap<String, u16>,
    next_address: u16,
}

impl SymbolLookup {
    pub fn new() -> Self {
        let mut lookup = Self {
            symbol_table: HashMap::new(),
            next_address: 0,
        };

        // Populate with R0 through R15
        for i in 0..16 {
            lookup.add_variable(format!("R{}", i));
        }

        lookup
    }

    pub fn set_line(&mut self, symbol: String, line_number: u16) {
        // Only add the symbol if it doesn't already exist
        if self.symbol_table.contains_key(&symbol) {
            return;
        }

        self.symbol_table.insert(symbol, line_number);
    }

    pub fn add_variable(&mut self, symbol: String) {
        if self.symbol_table.contains_key(&symbol) {
            return;
        }

        self.symbol_table.insert(symbol, self.next_address);
        self.next_address += 1;
    }

    pub fn get(&self, symbol: &str) -> Option<&u16> {
        self.symbol_table.get(symbol)
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbol_table.contains_key(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_retrieve_symbol() {
        let mut symbol_lookup = SymbolLookup::new();
        symbol_lookup.add_variable("n".to_string());

        assert_eq!(symbol_lookup.get("n"), Some(&16));
    }

    #[test]
    fn sets_r0_through_r15() {
        let symbol_lookup = SymbolLookup::new();

        for i in 0..16 {
            assert_eq!(symbol_lookup.get(&format!("R{}", i)), Some(&i));
        }
    }

    #[test]
    fn does_not_add_duplicate_symbols() {
        let mut symbol_lookup = SymbolLookup::new();
        symbol_lookup.add_variable("LOOP".to_string());
        symbol_lookup.add_variable("LOOP".to_string());

        assert_eq!(symbol_lookup.get("LOOP"), Some(&16));
    }
}
