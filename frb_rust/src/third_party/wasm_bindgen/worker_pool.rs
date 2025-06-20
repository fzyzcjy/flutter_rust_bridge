//! Copied and modified from the wasm_bindgen raytrace-parallel example
//!
//! File: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs

use crate::misc::web_utils::script_path;
use crate::web_transfer::transfer_closure::TransferClosure;
use js_sys::{Array, Object, Reflect};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::iter::FromIterator;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BlobPropertyBag;
use web_sys::ErrorEvent;
use web_sys::MessageEvent;
use web_sys::{Blob, Url};
use web_sys::{Event, Worker};

#[wasm_bindgen]
pub struct WorkerPool {
    state: Rc<PoolState>,
    script_src: String,
    worker_js_preamble: String,
}

struct PoolState {
    idle_workers: RefCell<Vec<Worker>>,
    callback: Closure<dyn FnMut(Event)>,
    max_workers: Option<usize>,
    work_queue: RefCell<VecDeque<PendingWork>>,
    total_workers: AtomicUsize,
}

struct PendingWork {
    closure: TransferClosure<JsValue>,
}

#[derive(Debug)]
enum ExecuteResult {
    /// Work was executed on the returned worker
    Executed(Worker),
    /// Work was queued for later execution (when max workers reached)
    Queued,
}

#[wasm_bindgen]
impl WorkerPool {
    pub fn new(
        initial: Option<usize>,
        script_src: Option<String>,
        worker_js_preamble: Option<String>,
    ) -> Result<WorkerPool, JsValue> {
        Self::new_raw(
            initial.unwrap_or_else(get_wasm_hardware_concurrency),
            script_src.unwrap_or_else(|| script_path().expect("fail to get script path")),
            worker_js_preamble.unwrap_or_default(),
        )
    }

    pub fn new_with_max_workers(max_workers: usize) -> Result<WorkerPool, JsValue> {
        let initial = get_wasm_hardware_concurrency().min(max_workers);
        Self::new_internal(
            initial,
            Some(max_workers),
            script_path().expect("fail to get script path"),
            String::new(),
        )
    }

    /// Creates a new `WorkerPool` which immediately creates `initial` workers.
    ///
    /// The pool created here can be used over a long period of time, and it
    /// will be initially primed with `initial` workers. Currently workers are
    /// never released or gc'd until the whole pool is destroyed.
    ///
    /// # Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    #[wasm_bindgen(constructor)]
    pub fn new_raw(
        initial: usize,
        script_src: String,
        worker_js_preamble: String,
    ) -> Result<WorkerPool, JsValue> {
        Self::new_internal(initial, None, script_src, worker_js_preamble)
    }

    fn new_internal(
        initial: usize,
        max_workers: Option<usize>,
        script_src: String,
        worker_js_preamble: String,
    ) -> Result<WorkerPool, JsValue> {
        let pool = WorkerPool {
            script_src,
            state: Rc::new(PoolState {
                idle_workers: RefCell::new(Vec::with_capacity(initial)),
                callback: Closure::new(|event: Event| {
                    if let Some(event) = event.dyn_ref::<MessageEvent>() {
                        crate::console_error!("Dropped data:: {:?}", event.data());
                    } else if let Some(event) = event.dyn_ref::<ErrorEvent>() {
                        crate::console_error!("Failed to initialize: {}", event.message());
                    }
                }),
                max_workers,
                work_queue: RefCell::new(VecDeque::new()),
                total_workers: AtomicUsize::new(0),
            }),
            worker_js_preamble,
        };
        for _ in 0..initial {
            let worker = pool.spawn()?;
            pool.state.push(worker);
            pool.state.total_workers.fetch_add(1, Ordering::SeqCst);
        }

        Ok(pool)
    }

    /// Unconditionally spawns a new worker
    ///
    /// The worker isn't registered with this `WorkerPool` but is capable of
    /// executing work for this wasm module.
    ///
    /// # Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    fn spawn(&self) -> Result<Worker, JsValue> {
        let worker_js_preamble = &self.worker_js_preamble;
        let script_src = &self.script_src;
        let script = format!(
            "{worker_js_preamble}
            importScripts('{script_src}');
            const FRB_ACTION_PANIC = 3;
            onmessage = event => {{
                let init = wasm_bindgen(...event.data).catch(err => {{
                    setTimeout(() => {{ throw err }})
                    throw err
                }})
                onmessage = async event => {{
                    await init
                    const [payload, ...transfer] = event.data
                    try {{
                        wasm_bindgen.receive_transfer_closure(payload, transfer)
                    }} catch (err) {{
                        if (transfer[0] && typeof transfer[0].postMessage === 'function') {{
                            // panic
                            transfer[0].postMessage([FRB_ACTION_PANIC, err.toString()])
                        }}
                        setTimeout(() => {{ throw err }})
                        postMessage(null)
                        throw err
                    }}
                }}
            }}",
        );
        let blob = Blob::new_with_blob_sequence_and_options(
            &Array::from_iter([JsValue::from(script)]).into(),
            BlobPropertyBag::new().type_("text/javascript"),
        )?;
        let url = Url::create_object_url_with_blob(&blob)?;
        let worker: Worker = Worker::new(&url)?;

        // With a worker spun up send it the module/memory so it can start
        // instantiating the wasm module. Later it might receive further
        // messages about code to run on the wasm module.
        let module = wasm_bindgen::module();
        let memory = wasm_bindgen::memory();
        let wasm_init_object = Object::new();
        Reflect::set(
            &wasm_init_object,
            &JsValue::from_str("module_or_path"),
            &module,
        )?;
        Reflect::set(&wasm_init_object, &JsValue::from_str("memory"), &memory)?;
        worker.post_message(&Array::from_iter([wasm_init_object]))?;

        Ok(worker)
    }

    /// Fetches a worker from this pool, spawning one if necessary.
    ///
    /// This will attempt to pull an already-spawned web worker from our cache
    /// if one is available, otherwise it will spawn a new worker and return the
    /// newly spawned worker.
    ///
    /// # Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    fn try_get_worker(&self) -> Option<Worker> {
        self.state.idle_workers.borrow_mut().pop()
    }
    
    fn can_spawn_worker(&self) -> bool {
        match self.state.max_workers {
            Some(max) => self.state.total_workers.load(Ordering::SeqCst) < max,
            None => true,
        }
    }

    /// Executes the work `f` in a web worker, spawning a web worker if
    /// necessary.
    ///
    /// This will acquire a web worker and then send the closure `f` to the
    /// worker to execute. The worker won't be usable for anything else while
    /// `f` is executing, and no callbacks are registered for when the worker
    /// finishes.
    ///
    /// ## Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    // NOTE: It is originally named `execute`, but rename to align with crate `threadpool`
    fn execute_raw(&self, closure: TransferClosure<JsValue>) -> Result<ExecuteResult, JsValue> {
        let worker = self.try_get_worker()
            .or_else(|| self.try_spawn_worker());
        
        match worker {
            Some(worker) => {
                closure.apply(&worker)?;
                Ok(ExecuteResult::Executed(worker))
            }
            None if self.state.max_workers.is_some() => {
                self.state.work_queue.borrow_mut().push_back(PendingWork { closure });
                Ok(ExecuteResult::Queued)
            }
            None => Err("Failed to spawn worker".into()),
        }
    }
    
    fn try_spawn_worker(&self) -> Option<Worker> {
        if self.can_spawn_worker() {
            self.spawn().ok().map(|worker| {
                self.state.total_workers.fetch_add(1, Ordering::SeqCst);
                worker
            })
        } else {
            None
        }
    }

    /// Configures an `onmessage` callback for the `worker` specified for the
    /// web worker to be reclaimed and re-inserted into this pool when a message
    /// is received.
    ///
    /// Currently this `WorkerPool` abstraction is intended to execute one-off
    /// style work where the work itself doesn't send any notifications and
    /// when it's done the worker is ready to execute more work. This method is
    /// used for all spawned workers to ensure that when the work is finished
    /// the worker is reclaimed back into this pool.
    fn reclaim_on_message(&self, worker: &Worker) {
        let state = Rc::downgrade(&self.state);
        let worker2 = worker.clone();
        let reclaim_slot = Rc::new(RefCell::new(None));
        let slot2 = reclaim_slot.clone();
        let reclaim = Closure::<dyn FnMut(_)>::new(move |_: MessageEvent| {
            if let Some(state) = state.upgrade() {
                if let Some(pending_work) = state.work_queue.borrow_mut().pop_front() {
                    let _ = pending_work.closure.apply(&worker2);
                } else {
                    state.push(worker2.clone());
                }
            }
            *slot2.borrow_mut() = None;
        });
        worker.set_onmessage(Some(reclaim.as_ref().unchecked_ref()));
        *reclaim_slot.borrow_mut() = Some(reclaim);
    }
}

impl WorkerPool {
    /// Executes `f` in a web worker.
    ///
    /// This pool manages a set of web workers to draw from, and `f` will be
    /// spawned quickly into one if the worker is idle. If no idle workers are
    /// available then a new web worker will be spawned. If the maximum number
    /// of workers has been reached, the work will be queued for later execution.
    ///
    /// Once `f` returns the worker assigned to `f` is automatically reclaimed
    /// by this `WorkerPool`. This method provides no method of learning when
    /// `f` completes, and for that you'll need to use `run_notify`.
    ///
    /// ## Errors
    ///
    /// If an error happens while spawning a web worker or sending a message to
    /// a web worker, that error is returned. Work being queued due to worker
    /// limits is not considered an error.
    ///
    /// ## Transferrables
    /// Items put inside `transfer` will have their ownership transferred from
    /// the invoking JS scope to the target, rendering the value unusable in the original
    /// scope. (This is similar to a `FnOnce` closure in Rust terms, but does not statically
    /// move items out of scope.)
    ///
    /// Certain types in [js_sys] and [web_sys] are transferrable, for which [Send]
    /// can be unsafely implemented **only if** they are passed to the transferrables of
    /// a `post_message`. Examples are `Buffer`s, `MessagePort`s, etc...
    // NOTE: It is originally named `run`, but rename to align with crate `threadpool`
    pub fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        match self.execute_raw(closure)? {
            ExecuteResult::Executed(worker) => {
                self.reclaim_on_message(&worker);
            }
            ExecuteResult::Queued => {
                // Work was successfully queued for later execution
            }
        }
        Ok(())
    }
}

impl PoolState {
    fn push(&self, worker: Worker) {
        worker.set_onmessage(Some(self.callback.as_ref().unchecked_ref()));
        worker.set_onerror(Some(self.callback.as_ref().unchecked_ref()));
        let mut idle_workers = self.idle_workers.borrow_mut();
        for prev in idle_workers.iter() {
            let prev: &JsValue = prev;
            let worker: &JsValue = &worker;
            assert!(prev != worker);
        }
        idle_workers.push(worker);
    }
}

impl Default for WorkerPool {
    fn default() -> Self {
        Self::new(None, None, None).expect("fail to create WorkerPool")
    }
}

fn get_wasm_hardware_concurrency() -> usize {
    let mut key;
    let global_object = js_sys::global();
    let global = global_object.as_ref();
    key = wasm_bindgen::JsValue::from_str("navigator");
    let navigator = js_sys::Reflect::get(global, &key).unwrap();
    key = wasm_bindgen::JsValue::from_str("hardwareConcurrency");
    let hardware_concurrency = js_sys::Reflect::get(&navigator, &key).unwrap();
    hardware_concurrency.as_f64().unwrap() as usize
}
