#[cfg(test)]
mod tests {
    use crate::process_filter::ProcessFilter;

    fn base_filter() -> ProcessFilter {
        ProcessFilter::default()
    }

    #[test]
    fn default_filter_matches_everything() {
        let f = base_filter();
        assert!(f.matches("nginx", 1234, "root", 5.0, 100.0));
        assert!(f.is_empty());
    }

    #[test]
    fn name_filter_case_insensitive() {
        let f = ProcessFilter {
            name_contains: Some("NGINX".into()),
            ..Default::default()
        };
        assert!(f.matches("nginx", 1, "root", 0.0, 0.0));
        assert!(!f.matches("apache", 2, "root", 0.0, 0.0));
    }

    #[test]
    fn pid_filter_exact_match() {
        let f = ProcessFilter {
            pid: Some(42),
            ..Default::default()
        };
        assert!(f.matches("any", 42, "user", 0.0, 0.0));
        assert!(!f.matches("any", 99, "user", 0.0, 0.0));
    }

    #[test]
    fn user_filter_exact_match() {
        let f = ProcessFilter {
            user: Some("alice".into()),
            ..Default::default()
        };
        assert!(f.matches("proc", 1, "alice", 0.0, 0.0));
        assert!(!f.matches("proc", 1, "bob", 0.0, 0.0));
    }

    #[test]
    fn min_cpu_filter() {
        let f = ProcessFilter {
            min_cpu_percent: Some(10.0),
            ..Default::default()
        };
        assert!(f.matches("proc", 1, "root", 15.0, 0.0));
        assert!(!f.matches("proc", 1, "root", 5.0, 0.0));
    }

    #[test]
    fn min_mem_filter() {
        let f = ProcessFilter {
            min_mem_mb: Some(50.0),
            ..Default::default()
        };
        assert!(f.matches("proc", 1, "root", 0.0, 64.0));
        assert!(!f.matches("proc", 1, "root", 0.0, 32.0));
    }

    #[test]
    fn combined_filters_all_must_match() {
        let f = ProcessFilter {
            name_contains: Some("redis".into()),
            min_cpu_percent: Some(5.0),
            min_mem_mb: Some(20.0),
            ..Default::default()
        };
        // All criteria satisfied
        assert!(f.matches("redis-server", 100, "redis", 8.0, 30.0));
        // Name matches but CPU too low
        assert!(!f.matches("redis-server", 100, "redis", 2.0, 30.0));
        // CPU and mem ok but wrong name
        assert!(!f.matches("memcached", 100, "redis", 8.0, 30.0));
    }
}
