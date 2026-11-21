// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-sandboxing: service sandboxing component model.
//!
//! Real implementation: sandboxings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sandboxing {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceSandboxingError {
    EmptySandboxingName,
    DuplicateSandboxing(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceSandboxingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySandboxingName => write!(f, "sandboxing name must not be empty"),
            Self::DuplicateSandboxing(id) => write!(f, "duplicate sandboxing {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceSandboxingError {}

pub struct ServiceSandboxing {
    pub name: String,
    sandboxings: BTreeMap<String, Sandboxing>,
    events: BTreeMap<String, Event>,
}

impl ServiceSandboxing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sandboxings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sandboxing(&mut self, item: Sandboxing) -> Result<(), ServiceSandboxingError> {
        if item.name.trim().is_empty() {
            return Err(ServiceSandboxingError::EmptySandboxingName);
        }
        let id = item.id.clone();
        if self.sandboxings.contains_key(&id) {
            return Err(ServiceSandboxingError::DuplicateSandboxing(id.clone()));
        }
        self.sandboxings.insert(id, item);
        Ok(())
    }

    pub fn sandboxing_count(&self) -> usize {
        self.sandboxings.len()
    }

    pub fn get_sandboxing(&self, id: &str) -> Option<&Sandboxing> {
        self.sandboxings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceSandboxingError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceSandboxingError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sandboxing(id: &str, name: &str) -> Sandboxing {
        Sandboxing {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceSandboxing::new("ServiceSandboxing");
        assert_eq!(m.name, "ServiceSandboxing");
        assert_eq!(m.sandboxing_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sandboxing() {
        let mut m = ServiceSandboxing::new("ServiceSandboxing");
        m.add_sandboxing(sample_sandboxing("a1", "Alpha")).unwrap();
        assert_eq!(m.sandboxing_count(), 1);
        assert!(m.get_sandboxing("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceSandboxing::new("ServiceSandboxing");
        assert!(m.add_sandboxing(sample_sandboxing("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceSandboxing::new("ServiceSandboxing");
        m.add_sandboxing(sample_sandboxing("x", "One")).unwrap();
        assert!(m.add_sandboxing(sample_sandboxing("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceSandboxing::new("ServiceSandboxing");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

/// Module extracted from the SigmaOS-system monolith.
pub mod sandbox;
