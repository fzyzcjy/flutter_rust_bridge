use crate::web_transfer::transfer_closure::TransferClosure;
use std::cell::RefCell;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use web_sys::{BroadcastChannel, MessageEvent, Worker};

thread_local! {
    static REGISTRATION: RefCell<Registration> = RefCell::new(Registration::default());
}

pub(crate) fn mark_channel_created() {
    REGISTRATION.with(|state| state.borrow_mut().dirty = true);
}

pub(crate) fn dispatch_after_channel_registration(
    worker: Worker,
    task: TransferClosure<JsValue>,
) -> Result<(), JsValue> {
    let immediate = REGISTRATION.with(|state| {
        let mut state = state.borrow_mut();
        if !state.dirty && state.pending.is_empty() {
            return Ok::<_, JsValue>(Some((worker, task)));
        }

        if state.barrier.is_none() {
            state.barrier = Some(Barrier::new()?);
        }
        if state.dirty {
            state.epoch = state.epoch.checked_add(1).expect("Channel epoch overflow");
            state.dirty = false;
        }
        let epoch = state.epoch;
        state.pending.push_back(PendingDispatch {
            epoch,
            worker,
            task,
        });
        state.barrier.as_mut().unwrap().post(epoch)?;
        Ok(None)
    })?;

    if let Some((worker, task)) = immediate {
        task.apply(&worker)?;
    }
    Ok(())
}

fn confirm_registration(event: MessageEvent) {
    let epoch = event.data().as_f64().expect("Invalid channel epoch") as u32;
    let ready = REGISTRATION.with(|state| {
        let mut state = state.borrow_mut();
        let mut ready = Vec::new();
        while state
            .pending
            .front()
            .is_some_and(|task| task.epoch <= epoch)
        {
            ready.push(state.pending.pop_front().unwrap());
        }
        if state.pending.is_empty() {
            state.barrier.as_mut().unwrap().stop_retry();
        }
        ready
    });
    for PendingDispatch { worker, task, .. } in ready {
        task.apply(&worker).unwrap_throw();
    }
}

fn retry_registration() {
    REGISTRATION.with(|state| {
        let state = state.borrow();
        state
            .barrier
            .as_ref()
            .unwrap()
            .sender
            .post_message(&JsValue::from(state.epoch))
            .unwrap_throw();
    });
}

#[derive(Default)]
struct Registration {
    dirty: bool,
    epoch: u32,
    pending: VecDeque<PendingDispatch>,
    barrier: Option<Barrier>,
}

struct PendingDispatch {
    epoch: u32,
    worker: Worker,
    task: TransferClosure<JsValue>,
}

struct Barrier {
    _receiver: BroadcastChannel,
    sender: BroadcastChannel,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    on_retry: Closure<dyn FnMut()>,
    interval: Option<i32>,
}

impl Barrier {
    fn new() -> Result<Self, JsValue> {
        let name = format!(
            "__frb_registration_{}_{}",
            js_sys::Date::now(),
            js_sys::Math::random()
        );
        let receiver = BroadcastChannel::new(&name)?;
        let sender = BroadcastChannel::new(&name)?;
        let on_message = Closure::new(confirm_registration);
        receiver.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        Ok(Self {
            _receiver: receiver,
            sender,
            _on_message: on_message,
            on_retry: Closure::new(retry_registration),
            interval: None,
        })
    }

    fn post(&mut self, epoch: u32) -> Result<(), JsValue> {
        self.sender.post_message(&JsValue::from(epoch))?;
        if self.interval.is_none() {
            self.interval = Some(set_interval(self.on_retry.as_ref().unchecked_ref(), 10)?);
        }
        Ok(())
    }

    fn stop_retry(&mut self) {
        if let Some(interval) = self.interval.take() {
            clear_interval(interval);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval, catch)]
    fn set_interval(callback: &js_sys::Function, delay: i32) -> Result<i32, JsValue>;
    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(interval: i32);
}
