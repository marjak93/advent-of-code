pub mod util;

#[path = "2025/mod.rs"]
pub mod year2025;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tower_http::services::ServeDir;
use year2025::day9::{get_polygon, Point, Polygon, Rect};

// Messages from client to server
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "start")]
    Start { speed: u64, num_cores: usize },
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "set_speed")]
    SetSpeed { speed: u64 },
    #[serde(rename = "set_cores")]
    SetCores { num_cores: usize },
}

// Messages from server to client
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "init")]
    Init {
        polygon: PolygonData,
        max_cores: usize,
    },
    #[serde(rename = "update")]
    Update {
        workers: Vec<WorkerUpdate>,
        current_best: u64,
        checked_count: usize,
    },
    #[serde(rename = "complete")]
    Complete { result: u64, checked_count: usize },
    #[serde(rename = "status")]
    Status { running: bool, paused: bool },
}

#[derive(Clone, Serialize, Deserialize)]
struct PolygonData {
    edges: Vec<LineData>,
    bounding_box: (i32, i32, i32, i32),
}

#[derive(Clone, Serialize, Deserialize)]
struct WorkerUpdate {
    worker_id: usize,
    rect: RectData,
    area: u64,
    is_contained: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct LineData {
    p1: PointData,
    p2: PointData,
}

#[derive(Clone, Serialize, Deserialize)]
struct PointData {
    x: i32,
    y: i32,
}

#[derive(Clone, Serialize, Deserialize)]
struct RectData {
    p1: PointData,
    p2: PointData,
}

impl From<Point> for PointData {
    fn from(p: Point) -> Self {
        PointData { x: p.x, y: p.y }
    }
}

impl From<&Rect> for RectData {
    fn from(r: &Rect) -> Self {
        RectData {
            p1: r.p1.into(),
            p2: r.p2.into(),
        }
    }
}

impl From<&Polygon> for PolygonData {
    fn from(p: &Polygon) -> Self {
        PolygonData {
            edges: p
                .edges
                .iter()
                .map(|e| LineData {
                    p1: e.p1.into(),
                    p2: e.p2.into(),
                })
                .collect(),
            bounding_box: p.bounding_box(),
        }
    }
}

#[derive(Clone)]
struct AppState {
    polygon: Arc<Polygon>,
    algorithm_state: Arc<RwLock<AlgorithmState>>,
}

struct AlgorithmState {
    running: bool,
    paused: bool,
    speed: u64, // microseconds delay
    num_cores: usize,
    stop_signal: Arc<RwLock<bool>>,
    work_queue: Arc<tokio::sync::Mutex<std::collections::VecDeque<usize>>>,
    worker_count: Arc<std::sync::atomic::AtomicUsize>,
    // Shared state for dynamic worker spawning
    candidates: Option<Arc<Vec<(Rect, u64)>>>,
    total_checked: Option<Arc<std::sync::atomic::AtomicUsize>>,
    best_area: Option<Arc<std::sync::atomic::AtomicU64>>,
    worker_states: Option<Arc<std::sync::Mutex<Vec<Option<(Rect, u64, bool)>>>>>,
}

impl AlgorithmState {
    fn new() -> Self {
        Self {
            running: false,
            paused: false,
            speed: 10000, // 10ms default - slow start
            num_cores: 1,
            stop_signal: Arc::new(RwLock::new(false)),
            work_queue: Arc::new(tokio::sync::Mutex::new(std::collections::VecDeque::new())),
            worker_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            candidates: None,
            total_checked: None,
            best_area: None,
            worker_states: None,
        }
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    println!("[WS] New WebSocket connection");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    println!("[WS] WebSocket connection established");
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    // Send initial polygon data with max available cores
    let max_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let init_msg = ServerMessage::Init {
        polygon: state.polygon.as_ref().into(),
        max_cores,
    };
    println!(
        "[WS] Sending initial polygon data (max_cores: {})",
        max_cores
    );
    let _ = tx.send(init_msg);

    // Spawn task to send messages to client
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages from client
    let tx_clone = tx.clone();
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                handle_client_message(client_msg, &state_clone, &tx_clone).await;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            println!("[WS] Send task completed, aborting receive task");
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            println!("[WS] Receive task completed, aborting send task");
            send_task.abort();
        },
    }
    println!("[WS] WebSocket connection closed");
}

async fn handle_client_message(
    msg: ClientMessage,
    state: &AppState,
    tx: &mpsc::UnboundedSender<ServerMessage>,
) {
    println!("[MSG] Received client message: {:?}", msg);
    match msg {
        ClientMessage::Start { speed, num_cores } => {
            let mut alg_state = state.algorithm_state.write().await;

            if alg_state.running {
                println!("[MSG] Algorithm already running, ignoring start request");
                return; // Already running
            }

            println!(
                "[MSG] Starting algorithm with speed: {} microseconds, num_cores: {}",
                speed, num_cores
            );
            alg_state.running = true;
            alg_state.paused = false;
            alg_state.speed = speed;
            alg_state.num_cores = num_cores;
            *alg_state.stop_signal.write().await = false; // Reset stop signal

            let status_msg = ServerMessage::Status {
                running: true,
                paused: false,
            };
            let _ = tx.send(status_msg);

            // Spawn algorithm thread
            let tx_clone = tx.clone();
            let polygon = state.polygon.clone();
            let alg_state_clone = state.algorithm_state.clone();

            tokio::spawn(async move {
                run_algorithm(polygon, alg_state_clone, tx_clone).await;
            });
        }
        ClientMessage::Pause => {
            println!("[MSG] Pausing algorithm");
            let mut alg_state = state.algorithm_state.write().await;
            alg_state.paused = true;

            let status_msg = ServerMessage::Status {
                running: alg_state.running,
                paused: true,
            };
            let _ = tx.send(status_msg);
        }
        ClientMessage::Resume => {
            println!("[MSG] Resuming algorithm");
            let mut alg_state = state.algorithm_state.write().await;
            alg_state.paused = false;

            let status_msg = ServerMessage::Status {
                running: alg_state.running,
                paused: false,
            };
            let _ = tx.send(status_msg);
        }
        ClientMessage::Stop => {
            println!("[MSG] Stopping algorithm");
            let mut alg_state = state.algorithm_state.write().await;
            *alg_state.stop_signal.write().await = true;
            alg_state.running = false;
            alg_state.paused = false;

            // Send a status update to confirm stop
            let status_msg = ServerMessage::Status {
                running: false,
                paused: false,
            };
            let _ = tx.send(status_msg);
        }
        ClientMessage::SetSpeed { speed } => {
            println!("[MSG] Setting speed to: {} microseconds", speed);
            let mut alg_state = state.algorithm_state.write().await;
            alg_state.speed = speed;
        }
        ClientMessage::SetCores { num_cores } => {
            println!("[MSG] Dynamically changing cores to: {}", num_cores);
            let alg_state = state.algorithm_state.read().await;

            if !alg_state.running {
                println!("[MSG] Not running, just updating core count");
                drop(alg_state);
                let mut alg_state_write = state.algorithm_state.write().await;
                alg_state_write.num_cores = num_cores;
                return;
            }

            let current_cores = alg_state
                .worker_count
                .load(std::sync::atomic::Ordering::Relaxed);
            println!(
                "[MSG] Current workers: {}, target: {}",
                current_cores, num_cores
            );

            if num_cores > current_cores {
                // Spawn additional workers
                println!(
                    "[MSG] Spawning {} additional workers",
                    num_cores - current_cores
                );

                // Get shared state needed for spawning
                let candidates = alg_state.candidates.clone();
                let total_checked = alg_state.total_checked.clone();
                let best_area = alg_state.best_area.clone();
                let worker_states = alg_state.worker_states.clone();
                let work_queue = alg_state.work_queue.clone();
                let worker_count = alg_state.worker_count.clone();
                let polygon = state.polygon.clone();
                let alg_state_for_workers = state.algorithm_state.clone();

                // Check if we have the necessary shared state
                if let (
                    Some(candidates),
                    Some(total_checked),
                    Some(best_area),
                    Some(worker_states),
                ) = (candidates, total_checked, best_area, worker_states)
                {
                    // Update worker count first
                    worker_count.store(num_cores, std::sync::atomic::Ordering::Relaxed);

                    // Spawn new workers
                    for worker_id in current_cores..num_cores {
                        let alg_state_clone = alg_state_for_workers.clone();
                        let polygon_clone = polygon.clone();
                        let candidates_clone = candidates.clone();
                        let total_checked_clone = total_checked.clone();
                        let best_area_clone = best_area.clone();
                        let worker_states_clone = worker_states.clone();
                        let work_queue_clone = work_queue.clone();
                        let worker_count_clone = worker_count.clone();

                        tokio::task::spawn_blocking(move || {
                            println!(
                                "[Worker {}] Started dynamically, pulling work from queue",
                                worker_id
                            );

                            loop {
                                // Check if this worker should exit (worker count reduced)
                                let current_worker_count =
                                    worker_count_clone.load(std::sync::atomic::Ordering::Relaxed);
                                if worker_id >= current_worker_count {
                                    println!(
                                        "[Worker {}] Exiting due to worker count reduction to {}",
                                        worker_id, current_worker_count
                                    );
                                    return;
                                }

                                // Get next work item from queue
                                let idx = {
                                    let runtime = tokio::runtime::Handle::current();
                                    runtime.block_on(async {
                                        work_queue_clone.lock().await.pop_front()
                                    })
                                };

                                let idx = match idx {
                                    Some(i) => i,
                                    None => {
                                        println!(
                                            "[Worker {}] No more work in queue, exiting",
                                            worker_id
                                        );
                                        return;
                                    }
                                };

                                let (rect, area) = &candidates_clone[idx];

                                // Check if we should stop
                                let should_stop = {
                                    let state = alg_state_clone.blocking_read();
                                    let stop_sig = state.stop_signal.blocking_read();
                                    *stop_sig
                                };

                                if should_stop {
                                    println!("[Worker {}] Stop signal received", worker_id);
                                    return;
                                }

                                // Wait while paused
                                loop {
                                    let (is_paused, should_stop) = {
                                        let state = alg_state_clone.blocking_read();
                                        let stop_sig = state.stop_signal.blocking_read();
                                        (state.paused, *stop_sig)
                                    };

                                    if should_stop {
                                        println!(
                                            "[Worker {}] Stop signal received while paused",
                                            worker_id
                                        );
                                        return;
                                    }

                                    if !is_paused {
                                        break;
                                    }

                                    std::thread::sleep(Duration::from_millis(50));
                                }

                                let is_contained = polygon_clone.can_contain_rect(rect);
                                total_checked_clone
                                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                                if is_contained {
                                    best_area_clone
                                        .fetch_max(*area, std::sync::atomic::Ordering::Relaxed);
                                }

                                // Update worker state for batched sending
                                {
                                    let mut states = worker_states_clone.lock().unwrap();
                                    // Ensure the worker_states vec is large enough
                                    while states.len() <= worker_id {
                                        states.push(None);
                                    }
                                    states[worker_id] = Some((*rect, *area, is_contained));
                                }

                                // Apply speed control
                                let speed = alg_state_clone.blocking_read().speed;
                                if speed > 0 {
                                    std::thread::sleep(Duration::from_micros(speed));
                                }

                                // If we found a valid rectangle and it's the largest possible, we can stop
                                let current_best =
                                    best_area_clone.load(std::sync::atomic::Ordering::Relaxed);
                                if is_contained && current_best == *area {
                                    println!(
                                        "[Worker {}] Found optimal solution: {}",
                                        worker_id, area
                                    );
                                    break;
                                }
                            }
                        });
                    }

                    println!(
                        "[MSG] Successfully spawned {} new workers",
                        num_cores - current_cores
                    );
                } else {
                    println!("[MSG] Cannot spawn workers: shared state not initialized");
                }
            } else if num_cores < current_cores {
                // Workers will naturally exit when they see the reduced worker_count
                println!(
                    "[MSG] Reducing worker count from {} to {}",
                    current_cores, num_cores
                );
                alg_state
                    .worker_count
                    .store(num_cores, std::sync::atomic::Ordering::Relaxed);
            }

            drop(alg_state);
            let mut alg_state_write = state.algorithm_state.write().await;
            alg_state_write.num_cores = num_cores;
        }
    }
}

async fn run_algorithm(
    polygon: Arc<Polygon>,
    alg_state: Arc<RwLock<AlgorithmState>>,
    tx: mpsc::UnboundedSender<ServerMessage>,
) {
    println!("[ALG] Starting algorithm execution");

    let num_cores = alg_state.read().await.num_cores;
    println!("[ALG] Using {} worker(s)", num_cores);

    // Get all candidates
    let input = crate::util::get_input(2025, 9);
    let points: Vec<Point> = year2025::day9::parse_input(&input);

    // Generate all candidate rectangles
    let mut candidates: Vec<(Rect, u64)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let rect = Rect { p1, p2 };
            let area = rect.area();
            candidates.push((rect, area));
        }
    }

    // Sort by area descending
    candidates.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    println!("[ALG] Total candidates: {}", candidates.len());

    // Initialize work queue with all candidate indices
    let work_queue = {
        let alg_state_read = alg_state.read().await;
        let mut queue = alg_state_read.work_queue.lock().await;
        queue.clear();
        for i in 0..candidates.len() {
            queue.push_back(i);
        }
        println!("[ALG] Work queue initialized with {} items", queue.len());
        alg_state_read.work_queue.clone()
    };

    let candidates_arc = Arc::new(candidates);
    let worker_count_global = alg_state.read().await.worker_count.clone();
    worker_count_global.store(num_cores, std::sync::atomic::Ordering::Relaxed);

    // Shared state for batched updates
    let total_checked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let best_area_global = Arc::new(std::sync::atomic::AtomicU64::new(0));
    // Use a larger initial size to accommodate dynamic workers
    let worker_states = Arc::new(std::sync::Mutex::new(vec![
        None::<(Rect, u64, bool)>;
        std::cmp::max(num_cores, 16) // Support up to 16 workers dynamically
    ]));

    // Store shared state in AlgorithmState for dynamic worker spawning
    {
        let mut alg_state_write = alg_state.write().await;
        alg_state_write.candidates = Some(candidates_arc.clone());
        alg_state_write.total_checked = Some(total_checked.clone());
        alg_state_write.best_area = Some(best_area_global.clone());
        alg_state_write.worker_states = Some(worker_states.clone());
    }

    // Spawn update sender task that sends at 60fps
    let tx_update = tx.clone();
    let worker_states_clone = worker_states.clone();
    let total_checked_clone = total_checked.clone();
    let best_area_clone = best_area_global.clone();
    let alg_state_update = alg_state.clone();

    let update_handle = tokio::spawn(async move {
        let frame_duration = Duration::from_micros(16667); // ~60fps
        loop {
            tokio::time::sleep(frame_duration).await;

            // Check if algorithm is still running
            let is_running = alg_state_update.read().await.running;
            if !is_running {
                break;
            }

            // Collect current worker states
            let states = worker_states_clone.lock().unwrap().clone();
            let mut workers = Vec::new();

            for (worker_id, state) in states.iter().enumerate() {
                if let Some((rect, area, is_contained)) = state {
                    workers.push(WorkerUpdate {
                        worker_id,
                        rect: rect.into(),
                        area: *area,
                        is_contained: *is_contained,
                    });
                }
            }

            if !workers.is_empty() {
                let update_msg = ServerMessage::Update {
                    workers,
                    current_best: best_area_clone.load(std::sync::atomic::Ordering::Relaxed),
                    checked_count: total_checked_clone.load(std::sync::atomic::Ordering::Relaxed),
                };
                let _ = tx_update.send(update_msg);
            }
        }
    });

    let mut worker_handles = Vec::new();

    for worker_id in 0..num_cores {
        let alg_state_clone = alg_state.clone();
        let polygon_clone = polygon.clone();
        let candidates_clone = candidates_arc.clone();
        let total_checked_clone = total_checked.clone();
        let best_area_clone = best_area_global.clone();
        let worker_states_clone = worker_states.clone();
        let work_queue_clone = work_queue.clone();
        let worker_count_clone = worker_count_global.clone();

        let handle = tokio::task::spawn_blocking(move || {
            println!("[Worker {}] Started, pulling work from queue", worker_id);

            loop {
                // Check if this worker should exit (worker count reduced)
                let current_worker_count =
                    worker_count_clone.load(std::sync::atomic::Ordering::Relaxed);
                if worker_id >= current_worker_count {
                    println!(
                        "[Worker {}] Exiting due to worker count reduction to {}",
                        worker_id, current_worker_count
                    );
                    return;
                }

                // Get next work item from queue
                let idx = {
                    let runtime = tokio::runtime::Handle::current();
                    runtime.block_on(async { work_queue_clone.lock().await.pop_front() })
                };

                let idx = match idx {
                    Some(i) => i,
                    None => {
                        println!("[Worker {}] No more work in queue, exiting", worker_id);
                        return;
                    }
                };

                let (rect, area) = &candidates_clone[idx];

                // Check if we should stop
                let should_stop = {
                    let state = alg_state_clone.blocking_read();
                    let stop_sig = state.stop_signal.blocking_read();
                    *stop_sig
                };

                if should_stop {
                    println!("[Worker {}] Stop signal received", worker_id);
                    return;
                }

                // Wait while paused
                loop {
                    let (is_paused, should_stop) = {
                        let state = alg_state_clone.blocking_read();
                        let stop_sig = state.stop_signal.blocking_read();
                        (state.paused, *stop_sig)
                    };

                    if should_stop {
                        println!("[Worker {}] Stop signal received while paused", worker_id);
                        return;
                    }

                    if !is_paused {
                        break;
                    }

                    std::thread::sleep(Duration::from_millis(50));
                }

                let is_contained = polygon_clone.can_contain_rect(rect);
                total_checked_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if is_contained {
                    best_area_clone.fetch_max(*area, std::sync::atomic::Ordering::Relaxed);
                }

                // Update worker state for batched sending
                {
                    let mut states = worker_states_clone.lock().unwrap();
                    states[worker_id] = Some((*rect, *area, is_contained));
                }

                // Apply speed control
                let speed = alg_state_clone.blocking_read().speed;
                if speed > 0 {
                    std::thread::sleep(Duration::from_micros(speed));
                }

                // If we found a valid rectangle and it's the largest possible, we can stop
                let current_best = best_area_clone.load(std::sync::atomic::Ordering::Relaxed);
                if is_contained && current_best == *area {
                    println!("[Worker {}] Found optimal solution: {}", worker_id, area);
                    break;
                }
            }
        });

        worker_handles.push(handle);
    }

    // Wait for all workers to complete
    for (i, handle) in worker_handles.into_iter().enumerate() {
        if let Err(e) = handle.await {
            println!("[ALG] Worker {} failed: {:?}", i, e);
        }
    }

    let final_result = best_area_global.load(std::sync::atomic::Ordering::Relaxed);
    let final_checked = total_checked.load(std::sync::atomic::Ordering::Relaxed);

    println!("[ALG] Algorithm completed with result: {}", final_result);

    // First, set running to false so update task exits
    {
        let mut alg_state_write = alg_state.write().await;
        alg_state_write.running = false;
    }

    // Wait for update task to finish before sending completion
    println!("[ALG] Waiting for update task to finish...");
    let _ = update_handle.await;
    println!("[ALG] Update task finished");

    // Now send completion message
    let complete_msg = ServerMessage::Complete {
        result: final_result,
        checked_count: final_checked,
    };
    let _ = tx.send(complete_msg);

    // Reset state for next run
    let mut alg_state_write = alg_state.write().await;
    alg_state_write.paused = false;
    *alg_state_write.stop_signal.write().await = false; // Reset stop signal for next run

    let status_msg = ServerMessage::Status {
        running: false,
        paused: false,
    };
    let _ = tx.send(status_msg);

    println!("[ALG] Algorithm fully cleaned up and ready for next run");
}

#[tokio::main]
async fn main() {
    println!("🎄 Advent of Code 2025 - Day 9 Part 2 Visualization 🎄");
    println!("Loading data...");

    let polygon = get_polygon();
    let polygon_arc = Arc::new(polygon);

    let state = AppState {
        polygon: polygon_arc,
        algorithm_state: Arc::new(RwLock::new(AlgorithmState::new())),
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/", ServeDir::new("static"))
        .with_state(state);

    let addr = "127.0.0.1:3000";
    println!("\n🌐 Starting web server at http://{}", addr);
    println!("📊 Open your browser to view the visualization");
    println!("Press Ctrl+C to stop");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
