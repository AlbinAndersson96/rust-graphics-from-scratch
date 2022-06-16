use gilrs::{Gilrs, Event};

pub struct InputHandle {
    gilrs : Gilrs
}

pub fn init() -> InputHandle {
    InputHandle { gilrs: Gilrs::new().unwrap() }
}

pub fn get_events(handle: &mut InputHandle) {
    while let Some(Event { id, event, time }) = handle.gilrs.next_event() {
        println!("{:?} New event from {}: {:?}", time, id, event);
    }
}
