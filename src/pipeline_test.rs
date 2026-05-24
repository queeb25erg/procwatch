#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::pipeline::{Pipeline, PipelineItem, StageResult};
    use crate::pipeline_manager::PipelineManager;

    fn make_item(pid: u32, name: &str, value: f64) -> PipelineItem {
        PipelineItem::new(pid, name, value)
    }

    #[test]
    fn test_pipeline_push_and_pop() {
        let mut pipe = Pipeline::new("test", 10);
        let item = make_item(1, "nginx", 42.0);
        assert!(pipe.push(item));
        assert_eq!(pipe.len(), 1);
        let popped = pipe.pop().unwrap();
        assert_eq!(popped.pid, 1);
        assert_eq!(popped.name, "nginx");
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipeline_capacity_drop() {
        let mut pipe = Pipeline::new("capped", 2);
        assert!(pipe.push(make_item(1, "a", 1.0)));
        assert!(pipe.push(make_item(2, "b", 2.0)));
        assert!(!pipe.push(make_item(3, "c", 3.0)));
        let (_, dropped) = pipe.stats();
        assert_eq!(dropped, 1);
    }

    #[test]
    fn test_pipeline_stats() {
        let mut pipe = Pipeline::new("stats", 10);
        pipe.push(make_item(1, "proc", 1.0));
        pipe.push(make_item(2, "proc", 2.0));
        pipe.pop();
        let (processed, dropped) = pipe.stats();
        assert_eq!(processed, 1);
        assert_eq!(dropped, 0);
    }

    #[test]
    fn test_item_with_label() {
        let item = make_item(10, "redis", 55.0).with_label("high-cpu");
        assert_eq!(item.label.as_deref(), Some("high-cpu"));
    }

    #[test]
    fn test_manager_push_pop() {
        let mut mgr = PipelineManager::new(100, Duration::from_secs(60));
        mgr.push("cpu", make_item(1, "nginx", 80.0));
        mgr.push("cpu", make_item(2, "redis", 50.0));
        let item = mgr.pop("cpu").unwrap();
        assert_eq!(item.pid, 1);
    }

    #[test]
    fn test_manager_apply_stage_pass() {
        let mut mgr = PipelineManager::new(100, Duration::from_secs(60));
        mgr.push("mem", make_item(1, "proc", 30.0));
        mgr.push("mem", make_item(2, "proc", 90.0));
        let passed = mgr.apply_stage("mem", |item| {
            if item.value > 80.0 {
                StageResult::Drop("too high".into())
            } else {
                StageResult::Pass
            }
        });
        assert_eq!(passed.len(), 1);
        assert_eq!(passed[0].value, 30.0);
    }

    #[test]
    fn test_manager_apply_stage_transform() {
        let mut mgr = PipelineManager::new(100, Duration::from_secs(60));
        mgr.push("io", make_item(5, "sshd", 10.0));
        let result = mgr.apply_stage("io", |_| StageResult::Transform("labeled".into()));
        assert_eq!(result[0].label.as_deref(), Some("labeled"));
    }

    #[test]
    fn test_manager_global_stats() {
        let mut mgr = PipelineManager::new(100, Duration::from_secs(60));
        mgr.push("a", make_item(1, "p", 1.0));
        mgr.push("b", make_item(2, "q", 2.0));
        mgr.pop("a");
        let (processed, _) = mgr.global_stats();
        assert_eq!(processed, 1);
    }
}
