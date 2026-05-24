use std::collections::HashMap;
use std::time::Duration;
use crate::pipeline::{Pipeline, PipelineItem, StageResult};

/// Manages multiple named pipelines
pub struct PipelineManager {
    pipelines: HashMap<String, Pipeline>,
    default_capacity: usize,
    max_age: Duration,
}

impl PipelineManager {
    pub fn new(default_capacity: usize, max_age: Duration) -> Self {
        Self {
            pipelines: HashMap::new(),
            default_capacity,
            max_age,
        }
    }

    pub fn get_or_create(&mut self, name: &str) -> &mut Pipeline {
        let capacity = self.default_capacity;
        self.pipelines
            .entry(name.to_string())
            .or_insert_with(|| Pipeline::new(name, capacity))
    }

    pub fn push(&mut self, pipeline: &str, item: PipelineItem) -> bool {
        let capacity = self.default_capacity;
        self.pipelines
            .entry(pipeline.to_string())
            .or_insert_with(|| Pipeline::new(pipeline, capacity))
            .push(item)
    }

    pub fn pop(&mut self, pipeline: &str) -> Option<PipelineItem> {
        self.pipelines.get_mut(pipeline)?.pop()
    }

    pub fn apply_stage<F>(&mut self, pipeline: &str, mut stage: F) -> Vec<PipelineItem>
    where
        F: FnMut(&PipelineItem) -> StageResult,
    {
        let Some(pipe) = self.pipelines.get_mut(pipeline) else {
            return vec![];
        };
        let mut passed = Vec::new();
        while let Some(item) = pipe.pop() {
            match stage(&item) {
                StageResult::Pass => passed.push(item),
                StageResult::Transform(label) => {
                    let mut transformed = item;
                    transformed.label = Some(label);
                    passed.push(transformed);
                }
                StageResult::Drop(_reason) => {}
            }
        }
        passed
    }

    pub fn flush_expired(&mut self) -> usize {
        let max_age = self.max_age;
        let mut total = 0;
        for pipe in self.pipelines.values_mut() {
            total += pipe.drain_expired(max_age).len();
        }
        total
    }

    pub fn pipeline_names(&self) -> Vec<&str> {
        self.pipelines.keys().map(|s| s.as_str()).collect()
    }

    pub fn global_stats(&self) -> (u64, u64) {
        self.pipelines
            .values()
            .fold((0, 0), |(ap, ad), p| {
                let (pp, pd) = p.stats();
                (ap + pp, ad + pd)
            })
    }
}
