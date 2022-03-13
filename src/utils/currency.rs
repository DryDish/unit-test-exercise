use super::units::CurrencyTypes;

/// Object designed to contain currency.
/// Contains a kind of currency and an amount
struct Currency {
    kind: CurrencyTypes,
    amount: i64,
}

impl Currency {
    /// Create a new object of type Currency.
    ///
    /// Must provide kind of currency.
    pub fn new(kind: CurrencyTypes) -> Self {
        return Self {
            kind: kind,
            amount: 0,
        };
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "kind: {:?}, amount: {}", self.kind, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_print() {
        let kroner = Currency::new(CurrencyTypes::DKK);
        println!("{}", kroner);
    }
}
