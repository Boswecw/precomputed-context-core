use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::enums::EventType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRecord {
    pub event_id: String,
    pub event_type: EventType,
    pub schema_version: String,
    pub emitted_at: String,
    pub emitter_service: String,
    pub repo_id: String,
    pub related_artifact_ids: Vec<String>,
    pub related_packet_ids: Vec<String>,
    pub source_refs: Vec<String>,
    pub causation_id: Option<String>,
    pub correlation_id: String,
    pub idempotency_key: String,
    pub event_payload: String,
}

impl EventRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.event_id.trim().is_empty() {
            return Err("event_id is required".into());
        }
        if self.schema_version.trim().is_empty() {
            return Err("schema_version is required".into());
        }
        if self.emitted_at.trim().is_empty() {
            return Err("emitted_at is required".into());
        }
        if self.emitter_service.trim().is_empty() {
            return Err("emitter_service is required".into());
        }
        if self.repo_id.trim().is_empty() {
            return Err("repo_id is required".into());
        }
        if self.correlation_id.trim().is_empty() {
            return Err("correlation_id is required".into());
        }
        if self.idempotency_key.trim().is_empty() {
            return Err("idempotency_key is required".into());
        }
        if self.event_payload.trim().is_empty() {
            return Err("event_payload is required".into());
        }
        Ok(())
    }

    fn coalescing_key(&self) -> String {
        let mut refs = self.source_refs.clone();
        refs.sort();
        format!("{}::{}::{}", self.repo_id, self.correlation_id, refs.join("|"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventProcessingDecision {
    Accepted,
    DuplicateIgnored,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBatch {
    pub repo_id: String,
    pub coalescing_key: String,
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Default)]
pub struct EventLedger {
    seen_idempotency_keys: HashSet<String>,
    pending: Vec<EventRecord>,
    poison_events: Vec<EventRecord>,
}

impl EventLedger {
    pub fn accept(&mut self, event: EventRecord) -> Result<EventProcessingDecision, String> {
        event.validate()?;

        if self.seen_idempotency_keys.contains(&event.idempotency_key) {
            return Ok(EventProcessingDecision::DuplicateIgnored);
        }

        self.seen_idempotency_keys
            .insert(event.idempotency_key.clone());
        self.pending.push(event);
        Ok(EventProcessingDecision::Accepted)
    }

    pub fn mark_poison(&mut self, event_id: &str) {
        if let Some(index) = self.pending.iter().position(|event| event.event_id == event_id) {
            let poisoned = self.pending.remove(index);
            self.poison_events.push(poisoned);
        }
    }

    pub fn poison_events(&self) -> &[EventRecord] {
        &self.poison_events
    }

    pub fn coalesce_pending(&mut self) -> Vec<EventBatch> {
        let mut grouped: HashMap<String, Vec<EventRecord>> = HashMap::new();

        for event in self.pending.drain(..) {
            grouped
                .entry(event.coalescing_key())
                .or_default()
                .push(event);
        }

        let mut batches = Vec::new();
        for (coalescing_key, mut events) in grouped {
            events.sort_by(|a, b| a.emitted_at.cmp(&b.emitted_at));
            let repo_id = events
                .first()
                .map(|event| event.repo_id.clone())
                .unwrap_or_default();

            batches.push(EventBatch {
                repo_id,
                coalescing_key,
                events,
            });
        }

        batches.sort_by(|a, b| a.coalescing_key.cmp(&b.coalescing_key));
        batches
    }
}
