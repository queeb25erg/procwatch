#[cfg(test)]
mod tests {
    use crate::label::{LabelSelector, LabelSet};
    use crate::label_manager::LabelManager;
    use crate::label_reporter::LabelReporter;

    fn make_set(pairs: &[(&str, &str)]) -> LabelSet {
        let mut s = LabelSet::new();
        for (k, v) in pairs {
            s.insert(*k, *v);
        }
        s
    }

    #[test]
    fn test_label_set_insert_and_get() {
        let mut s = LabelSet::new();
        s.insert("env", "prod");
        assert_eq!(s.get("env"), Some("prod"));
        assert_eq!(s.get("missing"), None);
    }

    #[test]
    fn test_label_set_merge_no_overwrite() {
        let mut base = make_set(&[("env", "prod"), ("region", "us-east")]);
        let extra = make_set(&[("env", "staging"), ("team", "ops")]);
        base.merge(&extra);
        // existing key should not be overwritten
        assert_eq!(base.get("env"), Some("prod"));
        assert_eq!(base.get("team"), Some("ops"));
    }

    #[test]
    fn test_selector_matches() {
        let labels = make_set(&[("env", "prod"), ("app", "nginx")]);
        let sel = LabelSelector::new().require("env", "prod");
        assert!(sel.matches(&labels));
    }

    #[test]
    fn test_selector_no_match() {
        let labels = make_set(&[("env", "staging")]);
        let sel = LabelSelector::new().require("env", "prod");
        assert!(!sel.matches(&labels));
    }

    #[test]
    fn test_empty_selector_matches_all() {
        let labels = make_set(&[("env", "prod")]);
        let sel = LabelSelector::new();
        assert!(sel.matches(&labels));
    }

    #[test]
    fn test_manager_get_labels_with_defaults() {
        let defaults = make_set(&[("region", "us-west")]);
        let mut mgr = LabelManager::new(defaults);
        mgr.set_labels("nginx", make_set(&[("app", "nginx")]));
        let labels = mgr.get_labels("nginx");
        assert_eq!(labels.get("region"), Some("us-west"));
        assert_eq!(labels.get("app"), Some("nginx"));
    }

    #[test]
    fn test_manager_find_matching() {
        let mut mgr = LabelManager::new(LabelSet::new());
        mgr.set_labels("nginx", make_set(&[("env", "prod")]));
        mgr.set_labels("redis", make_set(&[("env", "staging")]));
        let sel = LabelSelector::new().require("env", "prod");
        let matches = mgr.find_matching(&sel);
        assert_eq!(matches, vec!["nginx"]);
    }

    #[test]
    fn test_reporter_format() {
        let mut mgr = LabelManager::new(LabelSet::new());
        mgr.set_labels("nginx", make_set(&[("env", "prod"), ("app", "nginx")]));
        let report = LabelReporter::report_process(&mgr, "nginx");
        let formatted = report.format();
        assert!(formatted.contains("nginx"));
        assert!(formatted.contains("env=prod"));
        assert!(formatted.contains("app=nginx"));
    }

    #[test]
    fn test_manager_remove() {
        let mut mgr = LabelManager::new(LabelSet::new());
        mgr.set_labels("nginx", make_set(&[("env", "prod")]));
        assert_eq!(mgr.process_count(), 1);
        mgr.remove("nginx");
        assert_eq!(mgr.process_count(), 0);
    }
}
