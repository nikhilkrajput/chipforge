//! Event system for discrete-event simulation

use crate::value::Value;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Simulation time (in picoseconds or simulation time units)
pub type SimTime = u64;

/// Simulation event
#[derive(Debug, Clone)]
pub struct Event {
    /// Time when event occurs
    pub time: SimTime,
    /// Target signal/net ID
    pub target: usize,
    /// New value to assign
    pub value: Value,
    /// Priority (for delta cycles)
    pub priority: EventPriority,
}

/// Event priority for scheduling within same time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Inactive events (future time)
    Inactive = 0,
    /// Active events (current time, 0-delay)
    Active = 1,
    /// NBA (non-blocking assignment) events
    NonBlocking = 2,
    /// Monitor/strobe events (observe final values)
    Monitor = 3,
}

impl Event {
    /// Create a new event
    pub fn new(time: SimTime, target: usize, value: Value) -> Self {
        Self {
            time,
            target,
            value,
            priority: EventPriority::Active,
        }
    }

    /// Create event with specific priority
    pub fn with_priority(
        time: SimTime,
        target: usize,
        value: Value,
        priority: EventPriority,
    ) -> Self {
        Self {
            time,
            target,
            value,
            priority,
        }
    }
}

/// Wrapper for heap ordering (min-heap by time, then priority)
#[derive(Debug, Clone)]
struct EventWrapper(Event);

impl PartialEq for EventWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.time == other.0.time && self.0.priority == other.0.priority
    }
}

impl Eq for EventWrapper {}

impl PartialOrd for EventWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (BinaryHeap is max-heap by default)
        other
            .0
            .time
            .cmp(&self.0.time)
            .then_with(|| other.0.priority.cmp(&self.0.priority))
    }
}

/// Event queue for simulation
#[derive(Debug)]
pub struct EventQueue {
    /// Priority queue of events
    heap: BinaryHeap<EventWrapper>,
    /// Number of events processed
    events_processed: usize,
}

impl EventQueue {
    /// Create a new event queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            events_processed: 0,
        }
    }

    /// Schedule an event
    pub fn schedule(&mut self, event: Event) {
        self.heap.push(EventWrapper(event));
    }

    /// Get next event (removes from queue)
    pub fn pop(&mut self) -> Option<Event> {
        self.heap.pop().map(|wrapper| {
            self.events_processed += 1;
            wrapper.0
        })
    }

    /// Peek at next event without removing
    pub fn peek(&self) -> Option<&Event> {
        self.heap.peek().map(|wrapper| &wrapper.0)
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get number of pending events
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Get number of events processed
    pub fn events_processed(&self) -> usize {
        self.events_processed
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.heap.clear();
    }

    /// Get all events at a specific time
    pub fn get_events_at_time(&mut self, time: SimTime) -> Vec<Event> {
        let mut events = Vec::new();

        // Pop all events at this time
        while let Some(event) = self.peek() {
            if event.time == time {
                events.push(self.pop().unwrap());
            } else {
                break;
            }
        }

        events
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::BitValue;

    #[test]
    fn test_event_queue_ordering() {
        let mut queue = EventQueue::new();

        // Add events out of order
        queue.schedule(Event::new(100, 0, Value::Bit(BitValue::One)));
        queue.schedule(Event::new(50, 1, Value::Bit(BitValue::Zero)));
        queue.schedule(Event::new(200, 2, Value::Bit(BitValue::X)));

        // Should come out in time order
        assert_eq!(queue.pop().unwrap().time, 50);
        assert_eq!(queue.pop().unwrap().time, 100);
        assert_eq!(queue.pop().unwrap().time, 200);
    }

    #[test]
    fn test_event_priority() {
        let mut queue = EventQueue::new();

        // Add events at same time with different priorities
        queue.schedule(Event::with_priority(
            100,
            0,
            Value::Bit(BitValue::One),
            EventPriority::Monitor,
        ));
        queue.schedule(Event::with_priority(
            100,
            1,
            Value::Bit(BitValue::Zero),
            EventPriority::Active,
        ));
        queue.schedule(Event::with_priority(
            100,
            2,
            Value::Bit(BitValue::X),
            EventPriority::NonBlocking,
        ));

        // Should come out in priority order
        let e1 = queue.pop().unwrap();
        assert_eq!(e1.priority, EventPriority::Active);

        let e2 = queue.pop().unwrap();
        assert_eq!(e2.priority, EventPriority::NonBlocking);

        let e3 = queue.pop().unwrap();
        assert_eq!(e3.priority, EventPriority::Monitor);
    }

    #[test]
    fn test_event_queue_stats() {
        let mut queue = EventQueue::new();
        assert_eq!(queue.events_processed(), 0);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        queue.schedule(Event::new(10, 0, Value::Bit(BitValue::One)));
        queue.schedule(Event::new(20, 1, Value::Bit(BitValue::Zero)));

        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());

        queue.pop();
        assert_eq!(queue.events_processed(), 1);
        assert_eq!(queue.len(), 1);

        queue.pop();
        assert_eq!(queue.events_processed(), 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_get_events_at_time() {
        let mut queue = EventQueue::new();

        queue.schedule(Event::new(100, 0, Value::Bit(BitValue::One)));
        queue.schedule(Event::new(100, 1, Value::Bit(BitValue::Zero)));
        queue.schedule(Event::new(200, 2, Value::Bit(BitValue::X)));

        let events_100 = queue.get_events_at_time(100);
        assert_eq!(events_100.len(), 2);

        // Event at time 200 should still be in queue
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek().unwrap().time, 200);
    }
}
