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
use year2025::day9::{get_polygon, part2_visualize, Point, Polygon, Rect};

// Messages from client to server
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "start")]
    Start { speed: u64 },
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "set_speed")]
    SetSpeed { speed: u64 },
}

// Messages from server to client
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "init")]
    Init { polygon: PolygonData },
    #[serde(rename = "update")]
    Update {
        rect: RectData,
        area: u64,
        is_contained: bool,
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

#[derive(Clone)]
struct AlgorithmState {
    running: bool,
    paused: bool,
    speed: u64, // microseconds delay
    stop_signal: Arc<RwLock<bool>>,
}

impl AlgorithmState {
    fn new() -> Self {
        Self {
            running: false,
            paused: false,
            speed: 10000, // 10ms default - slow start
            stop_signal: Arc::new(RwLock::new(false)),
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

    // Send initial polygon data
    let init_msg = ServerMessage::Init {
        polygon: state.polygon.as_ref().into(),
    };
    println!("[WS] Sending initial polygon data");
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
        ClientMessage::Start { speed } => {
            let mut alg_state = state.algorithm_state.write().await;

            if alg_state.running {
                println!("[MSG] Algorithm already running, ignoring start request");
                return; // Already running
            }

            println!(
                "[MSG] Starting algorithm with speed: {} microseconds",
                speed
            );
            alg_state.running = true;
            alg_state.paused = false;
            alg_state.speed = speed;
            *alg_state.stop_signal.write().await = false;

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
    }
}

async fn run_algorithm(
    _polygon: Arc<Polygon>,
    alg_state: Arc<RwLock<AlgorithmState>>,
    tx: mpsc::UnboundedSender<ServerMessage>,
) {
    println!("[ALG] Starting algorithm execution");
    let mut checked_count = 0;

    // Clone for the blocking task
    let tx_clone = tx.clone();
    let alg_state_clone = alg_state.clone();

    // Run the algorithm in a blocking task
    let result = tokio::task::spawn_blocking(move || {
        part2_visualize(|rect, area, is_contained, current_best| {
            checked_count += 1;

            // Check if we should stop - avoid holding lock too long
            let should_stop = {
                let state = alg_state_clone.blocking_read();
                let stop_sig = state.stop_signal.blocking_read();
                *stop_sig
            };

            if should_stop {
                println!("[ALG] Stop signal received at count {}", checked_count);
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
                        "[ALG] Stop signal received while paused at count {}",
                        checked_count
                    );
                    return;
                }

                if !is_paused {
                    break;
                }

                std::thread::sleep(Duration::from_millis(50));
            }

            let update_msg = ServerMessage::Update {
                rect: rect.into(),
                area,
                is_contained,
                current_best,
                checked_count,
            };

            let _ = tx_clone.send(update_msg);

            // Apply speed control
            let speed = alg_state_clone.blocking_read().speed;
            if speed > 0 {
                std::thread::sleep(Duration::from_micros(speed));
            }
        })
    })
    .await;

    if let Ok(result) = result {
        println!("[ALG] Algorithm completed with result: {}", result);
        let complete_msg = ServerMessage::Complete {
            result,
            checked_count,
        };
        let _ = tx.send(complete_msg);

        let mut alg_state = alg_state.write().await;
        alg_state.running = false;
        alg_state.paused = false;

        let status_msg = ServerMessage::Status {
            running: false,
            paused: false,
        };
        let _ = tx.send(status_msg);
    } else {
        println!("[ALG] Algorithm task failed or was cancelled");
        let mut alg_state = alg_state.write().await;
        alg_state.running = false;
        alg_state.paused = false;
    }
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
