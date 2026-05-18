use super::aggregate_registry::AggregateRegistry;

pub struct LowerConfig {
    pub aggregates: AggregateRegistry,
}

impl Default for LowerConfig {
    fn default() -> Self {
        Self {
            aggregates: AggregateRegistry::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_builtins() {
        let config = LowerConfig::default();
        assert!(config.aggregates.contains("COUNT"));
        assert!(!config.aggregates.contains("apoc.coll.count"));
    }

    #[test]
    fn test_custom_aggregate_registration() {
        let mut config = LowerConfig::default();
        config.aggregates.register("custom.agg");
        assert!(config.aggregates.contains("custom.agg"));
    }
}
