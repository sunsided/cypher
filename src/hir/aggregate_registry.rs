use std::collections::HashSet;

/// Registry of aggregate function names for the Cypher HIR lowering pass.
///
/// Names are stored and matched using the dot-joined form (e.g.
/// `"apoc.agg.percentiles"`). Backtick-escaped segments whose unescaped
/// text contains a literal dot (e.g. `` `apoc.agg`.percentiles ``) are
/// normalised to the same dot-joined form and will match the same registry
/// entry. If your codebase uses such names as both aggregates and
/// non-aggregates, the aggregate classification may produce false positives.
pub struct AggregateRegistry {
    names: HashSet<String>,
}

impl AggregateRegistry {
    pub fn register(&mut self, name: &str) -> &mut Self {
        self.names.insert(name.to_uppercase());
        self
    }

    pub fn contains(&self, qualified_name: &str) -> bool {
        self.names.contains(&qualified_name.to_uppercase())
    }
}

impl Default for AggregateRegistry {
    fn default() -> Self {
        let names = [
            "COUNT",
            "SUM",
            "AVG",
            "MIN",
            "MAX",
            "COLLECT",
            "PERCENTILE_CONT",
            "PERCENTILE_DISC",
            "STDEV",
            "STDEVP",
            "VAR",
            "VARP",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        Self { names }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_contains_builtins() {
        let r = AggregateRegistry::default();
        for name in &[
            "COUNT",
            "SUM",
            "AVG",
            "MIN",
            "MAX",
            "COLLECT",
            "PERCENTILE_CONT",
            "PERCENTILE_DISC",
            "STDEV",
            "STDEVP",
            "VAR",
            "VARP",
        ] {
            assert!(r.contains(name), "{name} should be a built-in aggregate");
        }
    }

    #[test]
    fn test_register_custom() {
        let mut r = AggregateRegistry::default();
        assert!(!r.contains("apoc.agg.percentiles"));
        r.register("apoc.agg.percentiles");
        assert!(r.contains("apoc.agg.percentiles"));
    }

    #[test]
    fn test_lookup_is_case_insensitive() {
        let r = AggregateRegistry::default();
        assert!(r.contains("count"));
        assert!(r.contains("Count"));
        assert!(r.contains("COUNT"));
    }

    #[test]
    fn test_qualified_name_not_in_default() {
        let r = AggregateRegistry::default();
        assert!(!r.contains("apoc.coll.count"));
    }
}
