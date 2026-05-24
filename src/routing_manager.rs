use std::sync::{Arc, Mutex};
use crate::routing::{Route, Router};
use crate::alert::Alert;
use crate::notifier::NotifierKind;

#[derive(Clone)]
pub struct RoutingManager {
    router: Arc<Mutex<Router>>,
}

impl RoutingManager {
    pub fn new() -> Self {
        RoutingManager {
            router: Arc::new(Mutex::new(Router::new())),
        }
    }

    pub fn register(&self, route: Route) {
        let mut r = self.router.lock().unwrap();
        r.add_route(route);
    }

    pub fn unregister(&self, id: &str) {
        let mut r = self.router.lock().unwrap();
        r.remove_route(id);
    }

    pub fn resolve_targets(&self, alert: &Alert) -> Vec<NotifierKind> {
        let r = self.router.lock().unwrap();
        r.targets_for(alert)
    }

    pub fn route_count(&self) -> usize {
        let r = self.router.lock().unwrap();
        r.route_count()
    }
}

impl Default for RoutingManager {
    fn default() -> Self {
        Self::new()
    }
}
