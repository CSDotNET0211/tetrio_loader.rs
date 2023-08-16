use std::any::Any;
use crate::enums::event_type::EventType;


struct Event {
	id: i32,
	frame: usize,
	_type: EventType,
	data: dyn Any,
}

trait IEvent {}
