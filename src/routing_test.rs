#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::routing::{Route, RouteMatch, Router};
    use crate::routing_manager::RoutingManager;
    use crate::alert::Alert;
    use crate::notifier::NotifierKind;

    fn make_alert(name: &str) -> Alert {
        let mut a = Alert::new(name, "cpu", 95.0);
        a.tags.insert("critical".to_string());
        a.labels.insert("env".to_string(), "prod".to_string());
        a
    }

    #[test]
    fn test_route_matches_any() {
        let alert = make_alert("nginx");
        let route = Route::new("r1", RouteMatch::Any, vec![NotifierKind::Webhook], 1);
        assert!(route.matches(&alert));
    }

    #[test]
    fn test_route_matches_process_name() {
        let alert = make_alert("nginx");
        let route = Route::new("r2", RouteMatch::ProcessName("nginx".to_string()), vec![NotifierKind::Webhook], 1);
        assert!(route.matches(&alert));
        let route2 = Route::new("r3", RouteMatch::ProcessName("redis".to_string()), vec![], 1);
        assert!(!route2.matches(&alert));
    }

    #[test]
    fn test_route_matches_tag() {
        let alert = make_alert("nginx");
        let route = Route::new("r4", RouteMatch::Tag("critical".to_string()), vec![NotifierKind::Webhook], 2);
        assert!(route.matches(&alert));
        let route2 = Route::new("r5", RouteMatch::Tag("info".to_string()), vec![], 1);
        assert!(!route2.matches(&alert));
    }

    #[test]
    fn test_route_matches_label() {
        let alert = make_alert("nginx");
        let route = Route::new("r6", RouteMatch::Label("env".to_string(), "prod".to_string()), vec![NotifierKind::Webhook], 1);
        assert!(route.matches(&alert));
        let route2 = Route::new("r7", RouteMatch::Label("env".to_string(), "staging".to_string()), vec![], 1);
        assert!(!route2.matches(&alert));
    }

    #[test]
    fn test_router_priority_ordering() {
        let mut router = Router::new();
        router.add_route(Route::new("low", RouteMatch::Any, vec![NotifierKind::Webhook], 1));
        router.add_route(Route::new("high", RouteMatch::Any, vec![NotifierKind::Webhook], 10));
        let alert = make_alert("svc");
        let resolved = router.resolve(&alert);
        assert_eq!(resolved[0].id, "high");
    }

    #[test]
    fn test_routing_manager_dedup_targets() {
        let mgr = RoutingManager::new();
        mgr.register(Route::new("a", RouteMatch::Any, vec![NotifierKind::Webhook], 1));
        mgr.register(Route::new("b", RouteMatch::Any, vec![NotifierKind::Webhook], 2));
        let alert = make_alert("app");
        let targets = mgr.resolve_targets(&alert);
        assert_eq!(targets.len(), 1);
    }

    #[test]
    fn test_routing_manager_remove_route() {
        let mgr = RoutingManager::new();
        mgr.register(Route::new("x", RouteMatch::Any, vec![NotifierKind::Webhook], 1));
        assert_eq!(mgr.route_count(), 1);
        mgr.unregister("x");
        assert_eq!(mgr.route_count(), 0);
    }
}
