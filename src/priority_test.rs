#[cfg(test)]
mod tests {
    use crate::priority::{Priority, PriorityManager};

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!(Priority::from_str("low"), Some(Priority::Low));
        assert_eq!(Priority::from_str("Medium"), Some(Priority::Medium));
        assert_eq!(Priority::from_str("HIGH"), Some(Priority::High));
        assert_eq!(Priority::from_str("critical"), Some(Priority::Critical));
        assert_eq!(Priority::from_str("unknown"), None);
    }

    #[test]
    fn test_priority_as_str() {
        assert_eq!(Priority::Low.as_str(), "low");
        assert_eq!(Priority::Critical.as_str(), "critical");
    }

    #[test]
    fn test_priority_weight() {
        assert!(Priority::Critical.weight() > Priority::High.weight());
        assert!(Priority::High.weight() > Priority::Medium.weight());
        assert!(Priority::Medium.weight() > Priority::Low.weight());
    }

    #[test]
    fn test_manager_resolve_default() {
        let mut manager = PriorityManager::new();
        let p = manager.resolve("nginx");
        assert_eq!(p, Priority::Low);
    }

    #[test]
    fn test_manager_add_and_resolve() {
        let mut manager = PriorityManager::new();
        manager.add_rule("nginx", Priority::High);
        assert_eq!(manager.resolve("nginx"), Priority::High);
    }

    #[test]
    fn test_manager_resolves_highest_priority() {
        let mut manager = PriorityManager::new();
        manager.add_rule("nginx", Priority::Medium);
        manager.add_rule("nginx", Priority::Critical);
        manager.add_rule("nginx", Priority::High);
        assert_eq!(manager.resolve("nginx"), Priority::Critical);
    }

    #[test]
    fn test_manager_remove_rules() {
        let mut manager = PriorityManager::new();
        manager.add_rule("nginx", Priority::Critical);
        manager.remove_rules_for("nginx");
        assert_eq!(manager.resolve("nginx"), Priority::Low);
    }

    #[test]
    fn test_manager_cache_invalidated_on_add() {
        let mut manager = PriorityManager::new();
        manager.add_rule("sshd", Priority::Medium);
        let _ = manager.resolve("sshd"); // populate cache
        manager.add_rule("sshd", Priority::Critical);
        assert_eq!(manager.resolve("sshd"), Priority::Critical);
    }

    #[test]
    fn test_manager_all_rules() {
        let mut manager = PriorityManager::new();
        manager.add_rule("nginx", Priority::High);
        manager.add_rule("sshd", Priority::Low);
        assert_eq!(manager.all_rules().len(), 2);
    }
}
