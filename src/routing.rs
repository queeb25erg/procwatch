use std::collections::HashMap;
use crate::alert::Alert;
use crate::notifier::NotifierKind;

#[derive(Debug, Clone, PartialEq)]
pub enum RouteMatch {
    Tag(String),
    Label(String, String),
    ProcessName(String),
    Any,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub id: String,
    pub matcher: RouteMatch,
    pub targets: Vec<NotifierKind>,
    pub priority: u8,
}

impl Route {
    pub fn new(id: &str, matcher: RouteMatch, targets: Vec<NotifierKind>, priority: u8) -> Self {
        Route {
            id: id.to_string(),
            matcher,
            targets,
            priority,
        }
    }

    pub fn matches(&self, alert: &Alert) -> bool {
        match &self.matcher {
            RouteMatch::Any => true,
            RouteMatch::ProcessName(name) => alert.process_name == *name,
            RouteMatch::Tag(tag) => alert.tags.contains(tag),
            RouteMatch::Label(key, value) => {
                alert.labels.get(key).map(|v| v == value).unwrap_or(false)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
        self.routes.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn remove_route(&mut self, id: &str) {
        self.routes.retain(|r| r.id != id);
    }

    pub fn resolve(&self, alert: &Alert) -> Vec<&Route> {
        self.routes.iter().filter(|r| r.matches(alert)).collect()
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn targets_for(&self, alert: &Alert) -> Vec<NotifierKind> {
        let mut seen = std::collections::HashSet::new();
        let mut targets = Vec::new();
        for route in self.resolve(alert) {
            for t in &route.targets {
                if seen.insert(format!("{:?}", t)) {
                    targets.push(t.clone());
                }
            }
        }
        targets
    }
}
