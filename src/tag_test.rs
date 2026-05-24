#[cfg(test)]
mod tests {
    use crate::tag::TagSet;
    use std::collections::HashMap;

    #[test]
    fn test_insert_and_get() {
        let mut tags = TagSet::new();
        tags.insert("env", "prod");
        assert_eq!(tags.get("env"), Some("prod"));
        assert_eq!(tags.get("missing"), None);
    }

    #[test]
    fn test_contains() {
        let mut tags = TagSet::new();
        tags.insert("service", "api");
        assert!(tags.contains("service"));
        assert!(!tags.contains("region"));
    }

    #[test]
    fn test_remove() {
        let mut tags = TagSet::new();
        tags.insert("host", "server-01");
        let removed = tags.remove("host");
        assert_eq!(removed, Some("server-01".to_string()));
        assert!(!tags.contains("host"));
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut tags = TagSet::new();
        assert!(tags.is_empty());
        tags.insert("k", "v");
        assert_eq!(tags.len(), 1);
        assert!(!tags.is_empty());
    }

    #[test]
    fn test_merge_overwrites_existing() {
        let mut base = TagSet::new();
        base.insert("env", "staging");
        base.insert("region", "us-east");

        let mut extra = TagSet::new();
        extra.insert("env", "prod");
        extra.insert("team", "infra");

        base.merge(&extra);

        assert_eq!(base.get("env"), Some("prod"));
        assert_eq!(base.get("region"), Some("us-east"));
        assert_eq!(base.get("team"), Some("infra"));
        assert_eq!(base.len(), 3);
    }

    #[test]
    fn test_to_label_string_sorted() {
        let mut tags = TagSet::new();
        tags.insert("service", "api");
        tags.insert("env", "prod");
        tags.insert("region", "eu-west");
        let label = tags.to_label_string();
        assert_eq!(label, "env=prod,region=eu-west,service=api");
    }

    #[test]
    fn test_from_map() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "bar".to_string());
        let tags = TagSet::from_map(map);
        assert_eq!(tags.get("foo"), Some("bar"));
    }

    #[test]
    fn test_empty_label_string() {
        let tags = TagSet::new();
        assert_eq!(tags.to_label_string(), "");
    }
}
