use serde::{Deserialize, Serialize};
use crate::routing::{Route, RouteMatch};
use crate::notifier::NotifierKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub id: String,
    pub matcher: MatcherConfig,
    pub targets: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MatcherConfig {
    Any,
    ProcessName { name: String },
    Tag { tag: String },
    Label { key: String, value: String },
}

impl RouteConfig {
    pub fn to_route(&self) -> Route {
        let matcher = match &self.matcher {
            MatcherConfig::Any => RouteMatch::Any,
            MatcherConfig::ProcessName { name } => RouteMatch::ProcessName(name.clone()),
            MatcherConfig::Tag { tag } => RouteMatch::Tag(tag.clone()),
            MatcherConfig::Label { key, value } => RouteMatch::Label(key.clone(), value.clone()),
        };
        let targets = self.targets.iter().map(|t| match t.as_str() {
            "webhook" => NotifierKind::Webhook,
            _ => NotifierKind::Webhook,
        }).collect();
        Route::new(&self.id, matcher, targets, self.priority)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingConfig {
    pub routes: Vec<RouteConfig>,
}

impl RoutingConfig {
    pub fn to_routes(&self) -> Vec<Route> {
        self.routes.iter().map(|r| r.to_route()).collect()
    }

    pub fn example() -> Self {
        RoutingConfig {
            routes: vec![
                RouteConfig {
                    id: "default".to_string(),
                    matcher: MatcherConfig::Any,
                    targets: vec!["webhook".to_string()],
                    priority: 0,
                },
            ],
        }
    }
}
