use crate::event::Event;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, RwLock};

use nain_log as log;

#[macro_export]
macro_rules! subscribe_event {
    ($b:expr, $h:expr) => {
        $crate::subscribe_event($b, $h, 0)
    };
}

#[macro_export]
macro_rules! dispatch_event {
    ($b:expr, $e:expr) => {
        $crate::dispatch_event($b, $e)
    };
}

lazy_static! {
    static ref EVENT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref EVENT_ID_MAP: Mutex<HashMap<TypeId, usize>> = Mutex::new(HashMap::new());
    static ref EVENT_HANDLER_MAP: RwLock<HashMap<String, RwLock<HashMap<usize, Box<dyn Any + Send + Sync + 'static>>>>> =
        RwLock::new(HashMap::new());
}

pub struct EventBus {
    name: String,
}

struct EventHandlers<T: Event + ?Sized>(Vec<(usize, Box<dyn Fn(&mut T) + Send + Sync + 'static>)>);

impl<T: Event + ?Sized> Default for EventHandlers<T> {
    fn default() -> Self {
        EventHandlers(vec![])
    }
}

impl EventBus {
    pub fn new<S: Into<String>>(name: S) -> EventBus {
        let name = name.into();
        let mut map = EVENT_HANDLER_MAP
            .write()
            .expect("Failed to get write guard on handler map");

        if map.contains_key(&name) {
            panic!("Event bus named '{}' already exists!", name);
        }

        map.entry(name.clone())
            .or_insert_with(|| RwLock::new(HashMap::new()));

        EventBus { name }
    }
}

impl Drop for EventBus {
    fn drop(&mut self) {
        EVENT_HANDLER_MAP
            .write()
            .expect("Failed to get write guard on handler map")
            .remove(&self.name);
    }
}

pub fn dispatch_event<T: Event + ?Sized>(bus: &str, event: &mut T) {
    let event_id = 1;
    let map = EVENT_HANDLER_MAP
        .read()
        .expect("Failed to get read guard on handler map");

    if map.contains_key(bus) {
        let event_id_map = map
            .get(bus)
            .unwrap()
            .read()
            .expect("Failed to get read guard on event id map");

        println!("{:?} dispatch_event event_id", event_id);

        if let Some(handlers) = event_id_map.get(&event_id) {
            let handlers = handlers.downcast_ref::<EventHandlers<T>>().unwrap();

            println!("dispatch_event handlers");

            for handler in handlers.0.iter().rev() {
                handler.1(event);
            }
        }
    } else {
        log::warn!("Cannot dispatch event on invalid bus: '{}'", bus);
    }
}

pub fn subscribe_event<T: Event + ?Sized, H: Fn(&mut T) + Send + Sync + 'static>(
    bus: &str,
    handler: H,
    priority: usize,
) {
    let event_id = 1;
    let map = EVENT_HANDLER_MAP
        .read()
        .expect("Failed to get read guard on handler map");

    if map.contains_key(bus) {
        let mut event_id_map = map
            .get(bus)
            .unwrap()
            .write()
            .expect("Failed to get write guard on event id map");

        println!("{:?} subscribe_event event_id", event_id);

        let handlers = event_id_map
            .entry(event_id)
            .or_insert(Box::new(EventHandlers::<T>::default()))
            .downcast_mut::<EventHandlers<T>>()
            .unwrap();

        let pos = match handlers.0.binary_search_by(|probe| probe.0.cmp(&priority)) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        handlers.0.insert(pos, (priority, Box::new(handler)));
    } else {
        log::warn!("Cannot subscribe on invalid bus: '{}'", bus);
    }
}

fn get_event_id<T: Event + ?Sized>() -> usize {
    *EVENT_ID_MAP
        .lock()
        .expect("Failed to lock event id map")
        .entry(TypeId::of::<T>())
        .or_insert_with(|| EVENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}
