use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::any::Any;

/// A type-erased event that can hold any data
pub struct Event {
    /// The type ID of the event data
    type_id: u64,
    /// The data stored in the event
    data: Box<dyn Any + Send>,
}

impl Event {
    /// Creates a new event with the given data
    /// 
    /// # Arguments
    /// * `data` - The data to be stored in the event
    /// 
    /// # Returns
    /// A new event with the given data
    /// 
    /// # Type Parameters
    /// * `T` - The type of the data
    pub fn new<T: 'static + Send>(data: T) -> Self {
        Self {
            type_id: EventBus::get_type_id::<T>(),
            data: Box::new(data),
        }
    }

    /// Attempts to get a reference to the event data as type T
    /// 
    /// # Returns
    /// An optional reference to the data if it is of type T
    /// 
    /// # Type Parameters
    /// * `T` - The type of the data
    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    /// Returns the type ID of the event
    pub fn type_id(&self) -> u64 {
        self.type_id
    }
}

/// An event listener that can be registered for specific event types
pub struct EventListener<T> {
    /// The callback function that is called when the event is emitted
    callback: Box<dyn Fn(&T) + Send>,
}

/// The event bus that manages events and listeners
pub struct EventBus {
    /// A map of event type IDs to a list of listeners
    listeners: BTreeMap<u64, Vec<Box<dyn Any + Send>>>,
}

impl EventBus {
    /// Creates a new empty event bus
    /// 
    /// # Returns
    /// A new event bus with no listeners
    pub fn new() -> Self {
        Self {
            listeners: BTreeMap::new(),
        }
    }

    /// Register a listener for a specific event type
    /// 
    /// # Arguments
    /// * `listener` - The listener to be registered
    /// 
    /// # Type Parameters
    /// * `T` - The type of the event to listen for
    pub fn subscribe<T: 'static + Send>(&mut self, listener: EventListener<T>) {
        let type_id = Self::get_type_id::<T>();
        self.listeners
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }

    /// Emit an event to all registered listeners
    /// 
    /// # Arguments
    /// * `event` - The event to be emitted
    /// 
    /// # Type Parameters
    /// * `T` - The type of the event being emitted
    pub fn emit<T: 'static + Send>(&self, event: T) {
        let event = Event::new(event);
        let type_id = event.type_id();
        if let Some(listeners) = self.listeners.get(&type_id) {
            for listener in listeners {
                if let Some(listener) = listener.downcast_ref::<EventListener<T>>() {
                    if let Some(data) = event.get_data::<T>() {
                        (listener.callback)(data);
                    }
                }
            }
        }
    }

    /// Generate a unique ID for a type
    fn get_type_id<T: 'static>() -> u64 {
        let type_id = core::any::TypeId::of::<T>();
        // Use the type_id itself as the hash input by formatting it
        let type_str = format!("{:?}", type_id);
        
        // Use a simple FNV-1a hash
        const FNV_PRIME: u64 = 1099511628211;
        const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
        
        let mut hash = FNV_OFFSET_BASIS;
        for byte in type_str.as_bytes() {
            hash = hash.wrapping_mul(FNV_PRIME);
            hash ^= *byte as u64;
        }
        hash
    }
}

impl<T> EventListener<T> {
    /// Create a new event listener with the given callback
    /// 
    /// # Arguments
    /// * `callback` - The function to be called when the event is emitted
    /// 
    /// # Type Parameters
    /// * `F` - The type of the callback function
    /// 
    /// # Returns
    /// A new event listener with the given callback
    /// 
    /// # Constraints
    /// * `F` must be a function that takes a reference to type `T` and returns `()`
    pub fn new<F>(callback: F) -> Self 
    where
        F: Fn(&T) + Send + 'static 
    {
        Self {
            callback: Box::new(callback),
        }
    }
}