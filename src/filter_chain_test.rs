#[cfg(test)]
mod tests {
    use crate::filter_chain::{FilterChain, FilterRule};
    use crate::process_filter::ProcessFilter;
    use crate::priority::Priority;

    fn make_rule(name_pattern: &str, priority: Priority, label: &str) -> FilterRule {
        FilterRule::new(
            ProcessFilter::by_name(name_pattern),
            priority,
        )
        .with_label(label)
    }

    #[test]
    fn test_empty_chain_returns_none() {
        let chain = FilterChain::new();
        assert!(chain.evaluate("nginx", 1234).is_none());
    }

    #[test]
    fn test_single_rule_matches() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::Medium, "nginx-rule"));
        let result = chain.evaluate("nginx", 1234);
        assert!(result.is_some());
        assert_eq!(result.unwrap().label.as_deref(), Some("nginx-rule"));
    }

    #[test]
    fn test_no_match_returns_none() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::Medium, "nginx-rule"));
        assert!(chain.evaluate("apache", 9999).is_none());
    }

    #[test]
    fn test_priority_ordering() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::Low, "low-rule"));
        chain.add_rule(make_rule("nginx", Priority::High, "high-rule"));
        let result = chain.evaluate("nginx", 42);
        assert_eq!(result.unwrap().label.as_deref(), Some("high-rule"));
    }

    #[test]
    fn test_evaluate_all_returns_all_matches() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::Low, "low-rule"));
        chain.add_rule(make_rule("nginx", Priority::High, "high-rule"));
        let results = chain.evaluate_all("nginx", 42);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_remove_rule() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::Medium, "to-remove"));
        assert_eq!(chain.rule_count(), 1);
        let removed = chain.remove_rule("to-remove");
        assert!(removed);
        assert_eq!(chain.rule_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_rule() {
        let mut chain = FilterChain::new();
        let removed = chain.remove_rule("ghost");
        assert!(!removed);
    }

    #[test]
    fn test_clear() {
        let mut chain = FilterChain::new();
        chain.add_rule(make_rule("nginx", Priority::High, "r1"));
        chain.add_rule(make_rule("apache", Priority::Low, "r2"));
        chain.clear();
        assert_eq!(chain.rule_count(), 0);
    }
}
