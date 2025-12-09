// TypeScript-style Vue 3 Application for AoC 2025 Day 9 Part 2 Visualization
// This uses JSDoc comments for type hints since we're running in browser without a build step

const { createApp } = Vue;

/**
 * @typedef {Object} Point
 * @property {number} x
 * @property {number} y
 */

/**
 * @typedef {Object} Line
 * @property {Point} p1
 * @property {Point} p2
 */

/**
 * @typedef {Object} Polygon
 * @property {Line[]} edges
 * @property {[number, number, number, number]} bounding_box
 */

/**
 * @typedef {Object} Rect
 * @property {Point} p1
 * @property {Point} p2
 */

createApp({
  data() {
    return {
      /** @type {WebSocket | null} */
      ws: null,
      /** @type {Polygon | null} */
      polygon: null,
      /** @type {Map<number, Rect>} */
      workerRects: new Map(),
      /** @type {Map<number, boolean>} */
      workerContained: new Map(),
      /** @type {Map<number, number>} */
      workerAreas: new Map(),
      /** @type {Set<number>} */
      workersFoundBest: new Set(),
      /** @type {number} */
      checkedCount: 0,
      /** @type {number} */
      bestArea: 0,
      /** @type {boolean} */
      isRunning: false,
      /** @type {boolean} */
      isPaused: false,
      /** @type {number} */
      speedStep: 12, // 0-14, 15 steps from 100µs to 5s, default 1s
      /** @type {number} */
      numCores: 1,
      /** @type {number} */
      maxCores: 1,
      /** @type {string} */
      connectionStatus: "Connecting...",
      /** @type {string} */
      statusMessage: "Ready to start",
      /** @type {number} */
      reconnectAttempts: 0,
      /** @type {number} */
      maxReconnectAttempts: 5,

      // Canvas settings
      /** @type {HTMLCanvasElement[]} */
      canvases: [],
      /** @type {CanvasRenderingContext2D[]} */
      contexts: [],
      /** @type {number} */
      scale: 1,
      /** @type {number} */
      offsetX: 0,
      /** @type {number} */
      offsetY: 0,
      /** @type {number} */
      padding: 50,

      // Message log
      /** @type {Array<{time: string, content: string, type: string}>} */
      messageLog: [],
      /** @type {number} */
      maxLogEntries: 100,

      // Render throttling for 60fps
      /** @type {Set<number>} */
      workersNeedingRender: new Set(),
      /** @type {boolean} */
      renderScheduled: false,
      /** @type {number|null} */
      animationFrameId: null,

      // FPS tracking
      /** @type {number} */
      fps: 0,
      /** @type {number} */
      lastFrameTime: 0,
      /** @type {number[]} */
      frameTimes: [],
    };
  },
  computed: {
    connectionClass() {
      if (this.connectionStatus === "Connected") return "connected";
      if (this.connectionStatus === "Disconnected") return "disconnected";
      return "connecting";
    },
    statusClass() {
      if (this.isRunning && !this.isPaused) return "status-running";
      if (this.isPaused) return "status-paused";
      if (this.bestArea > 0 && !this.isRunning) return "status-complete";
      return "status-idle";
    },
    speed() {
      // 15 steps from 100µs to 5s
      const steps = [
        100, // 0: 100µs
        200, // 1: 200µs
        500, // 2: 500µs
        1000, // 3: 1ms
        2000, // 4: 2ms
        5000, // 5: 5ms
        10000, // 6: 10ms
        20000, // 7: 20ms
        50000, // 8: 50ms
        100000, // 9: 100ms
        200000, // 10: 200ms
        500000, // 11: 500ms
        1000000, // 12: 1s (default)
        2000000, // 13: 2s
        5000000, // 14: 5s
      ];
      return steps[this.speedStep];
    },
    speedLabel() {
      const micros = this.speed;
      if (micros < 1000) return `${micros}µs`;
      if (micros < 1000000) return `${micros / 1000}ms`;
      const seconds = micros / 1000000;
      return seconds >= 1 ? `${seconds}s` : `${seconds}s`;
    },
    delayMs() {
      return this.speedLabel;
    },
  },
  mounted() {
    this.connectWebSocket();
    window.addEventListener("resize", this.handleResize);
  },
  beforeUnmount() {
    if (this.ws) {
      this.ws.close();
    }
    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId);
    }
    window.removeEventListener("resize", this.handleResize);
  },
  methods: {
    connectWebSocket() {
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      const wsUrl = `${protocol}//${window.location.host}/ws`;

      console.log("[WS] Connecting to:", wsUrl);
      this.connectionStatus = "Connecting...";

      try {
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
          this.connectionStatus = "Connected";
          this.reconnectAttempts = 0;
          console.log("[WS] Connected successfully");
        };

        this.ws.onclose = (event) => {
          this.connectionStatus = "Disconnected";
          console.log("[WS] Connection closed:", event.code, event.reason);

          // Reset running state on disconnect
          this.isRunning = false;
          this.isPaused = false;

          // Attempt reconnect with exponential backoff
          if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = Math.min(
              1000 * Math.pow(2, this.reconnectAttempts - 1),
              10000
            );
            console.log(
              `[WS] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})...`
            );
            setTimeout(() => this.connectWebSocket(), delay);
          } else {
            this.statusMessage = "Connection lost. Please refresh the page.";
            console.error("[WS] Max reconnection attempts reached");
          }
        };

        this.ws.onerror = (error) => {
          console.error("[WS] WebSocket error:", error);
          this.connectionStatus = "Error";
        };

        this.ws.onmessage = (event) => {
          try {
            const msg = JSON.parse(event.data);
            this.handleMessage(msg);
          } catch (e) {
            console.error("[WS] Failed to parse message:", e, event.data);
          }
        };
      } catch (e) {
        console.error("[WS] Failed to create WebSocket:", e);
        this.connectionStatus = "Error";
      }
    },

    handleMessage(msg) {
      console.log("[MSG] Received:", msg.type);

      // Add message to log
      this.addLogEntry(msg);

      switch (msg.type) {
        case "init":
          this.polygon = msg.polygon;
          this.maxCores = msg.max_cores;
          this.numCores = Math.min(this.numCores, this.maxCores);
          console.log(
            "[MSG] Polygon initialized with",
            msg.polygon.edges.length,
            "edges, max_cores:",
            msg.max_cores
          );
          this.createCanvases();
          this.setupViewport();
          this.renderAll();
          break;

        case "update":
          // Handle batched worker updates
          msg.workers.forEach((worker) => {
            this.workerRects.set(worker.worker_id, worker.rect);
            this.workerContained.set(worker.worker_id, worker.is_contained);
            this.workerAreas.set(worker.worker_id, worker.area);

            // Mark worker as found best if it has the best area
            if (
              worker.is_contained &&
              worker.area === msg.current_best &&
              msg.current_best > 0
            ) {
              this.workersFoundBest.add(worker.worker_id);
            }

            this.scheduleRender(worker.worker_id);
          });
          this.bestArea = msg.current_best;
          this.checkedCount = msg.checked_count;
          break;

        case "complete":
          // Mark any workers that have the best area before clearing
          this.workerAreas.forEach((area, workerId) => {
            const isContained = this.workerContained.get(workerId);
            if (isContained && area === msg.result && msg.result > 0) {
              this.workersFoundBest.add(workerId);
            }
          });

          this.bestArea = msg.result;
          this.checkedCount = msg.checked_count;
          this.isRunning = false;
          this.isPaused = false;
          // Keep the last rectangles visible but clear their active state
          this.workerRects.clear();
          this.workerContained.clear();
          this.workerAreas.clear();
          this.statusMessage = `Complete! Final answer: ${msg.result.toLocaleString()}`;
          console.log("[MSG] Algorithm complete:", msg.result);
          this.renderAll();
          break;

        case "status":
          this.isRunning = msg.running;
          this.isPaused = msg.paused;
          console.log(
            "[MSG] Status update - running:",
            msg.running,
            "paused:",
            msg.paused
          );

          if (msg.running && !msg.paused) {
            this.statusMessage = "Running algorithm...";
          } else if (msg.paused) {
            this.statusMessage = "Paused";
          } else if (!msg.running) {
            if (this.bestArea > 0) {
              this.statusMessage = `Stopped at ${this.checkedCount.toLocaleString()} checks`;
            } else {
              this.statusMessage = "Ready to start";
            }
          }
          break;

        default:
          console.warn("[MSG] Unknown message type:", msg.type);
      }
    },

    sendMessage(msg) {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        console.log("[WS] Sending:", msg);
        this.ws.send(JSON.stringify(msg));
      } else {
        console.error("[WS] Cannot send message, WebSocket not open");
      }
    },

    start() {
      if (this.isRunning) {
        console.log("[UI] Already running, ignoring start");
        return;
      }

      // Recreate canvases if the number of cores has changed
      if (this.canvases.length !== this.numCores) {
        console.log(`[UI] Recreating canvases for ${this.numCores} cores`);
        this.createCanvases();
      }

      // Reset state
      this.checkedCount = 0;
      this.bestArea = 0;
      this.workerRects.clear();
      this.workerContained.clear();
      this.workerAreas.clear();
      this.workersFoundBest.clear();
      this.renderAll();

      // Speed is already in microseconds from computed property
      const speedMicros = this.speed;
      console.log(
        "[UI] Starting with speed:",
        this.speedLabel,
        "(",
        speedMicros,
        "microseconds), cores:",
        this.numCores
      );
      this.sendMessage({
        type: "start",
        speed: speedMicros,
        num_cores: this.numCores,
      });
      this.statusMessage = "Starting...";
    },

    pause() {
      console.log("[UI] Pausing");
      this.sendMessage({ type: "pause" });
    },

    resume() {
      console.log("[UI] Resuming");
      this.sendMessage({ type: "resume" });
    },

    stop() {
      console.log("[UI] Stopping");
      this.sendMessage({ type: "stop" });
      this.workerRects.clear();
      this.workerContained.clear();
      this.workerAreas.clear();
      this.workersFoundBest.clear();
      this.checkedCount = 0;
      this.bestArea = 0;
      this.statusMessage = "Stopped";
      this.renderAll();
    },

    setSpeed() {
      // Speed is already in microseconds from computed property
      const speedMicros = this.speed;
      console.log(
        "[UI] Setting speed to:",
        this.speedLabel,
        "(",
        speedMicros,
        "microseconds)"
      );
      if (this.isRunning) {
        this.sendMessage({ type: "set_speed", speed: speedMicros });
      }
    },

    createCanvases() {
      const container = this.$refs.canvasContainer;
      if (!container) return;

      // Clear existing canvases
      container.innerHTML = "";
      this.canvases = [];
      this.contexts = [];

      // Calculate grid layout
      const cols = Math.ceil(Math.sqrt(this.numCores));
      const rows = Math.ceil(this.numCores / cols);

      console.log(
        `[CANVAS] Creating ${this.numCores} canvases in ${rows}x${cols} grid`
      );

      // Create canvases
      for (let i = 0; i < this.numCores; i++) {
        const wrapper = document.createElement("div");
        wrapper.className = "canvas-wrapper";

        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");

        const label = document.createElement("div");
        label.className = "canvas-label";
        label.textContent = `Worker ${i}`;

        wrapper.appendChild(label);
        wrapper.appendChild(canvas);
        container.appendChild(wrapper);

        this.canvases.push(canvas);
        this.contexts.push(ctx);
      }

      // Set grid template
      container.style.display = "grid";
      container.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;
      container.style.gridTemplateRows = `repeat(${rows}, 1fr)`;
      container.style.gap = "10px";

      // Size canvases immediately after creation
      this.setupResponsiveCanvas();
    },

    setupResponsiveCanvas() {
      if (this.canvases.length === 0) return;

      const container = this.$refs.canvasContainer;
      if (!container) return;

      // Calculate per-canvas size based on container
      const cols = Math.ceil(Math.sqrt(this.numCores));
      const rows = Math.ceil(this.numCores / cols);

      const containerWidth = container.clientWidth;
      const containerHeight = container.clientHeight;

      // Account for gaps and padding
      const gap = 10;
      const padding = 40;

      const availableWidth = containerWidth - padding - (cols - 1) * gap;
      const availableHeight = containerHeight - padding - (rows - 1) * gap;

      let canvasWidth = availableWidth / cols;
      let canvasHeight = availableHeight / rows;

      // Maintain aspect ratio (4:3)
      const aspectRatio = 4 / 3;
      if (canvasWidth / canvasHeight > aspectRatio) {
        canvasWidth = canvasHeight * aspectRatio;
      } else {
        canvasHeight = canvasWidth / aspectRatio;
      }

      // Ensure minimum size
      canvasWidth = Math.max(canvasWidth, 150);
      canvasHeight = Math.max(canvasHeight, 112.5);

      this.canvases.forEach((canvas) => {
        canvas.width = canvasWidth;
        canvas.height = canvasHeight;
      });

      console.log(
        "[CANVAS] Resized",
        this.numCores,
        "canvases to:",
        Math.round(canvasWidth),
        "x",
        Math.round(canvasHeight)
      );

      if (this.polygon) {
        this.setupViewport();
        this.renderAll();
      }
    },

    handleResize() {
      this.setupResponsiveCanvas();
    },

    setupViewport() {
      if (!this.polygon || this.canvases.length === 0) return;

      const canvas = this.canvases[0]; // Use first canvas for calculations
      const [minX, maxX, minY, maxY] = this.polygon.bounding_box;
      const polyWidth = maxX - minX;
      const polyHeight = maxY - minY;

      const scaleX = (canvas.width - 2 * this.padding) / polyWidth;
      const scaleY = (canvas.height - 2 * this.padding) / polyHeight;
      this.scale = Math.min(scaleX, scaleY);

      const scaledWidth = polyWidth * this.scale;
      const scaledHeight = polyHeight * this.scale;
      this.offsetX = (canvas.width - scaledWidth) / 2 - minX * this.scale;
      this.offsetY = (canvas.height - scaledHeight) / 2 - minY * this.scale;

      console.log(
        "[RENDER] Viewport setup - scale:",
        this.scale,
        "offset:",
        this.offsetX,
        this.offsetY
      );
    },

    /** @param {Point} p */
    worldToScreen(p) {
      return {
        x: p.x * this.scale + this.offsetX,
        y: p.y * this.scale + this.offsetY,
      };
    },

    drawPolygon(ctx) {
      if (!this.polygon || !ctx) return;

      ctx.strokeStyle = "#ffffff";
      ctx.lineWidth = 2;
      ctx.beginPath();

      this.polygon.edges.forEach((edge, i) => {
        const p1 = this.worldToScreen(edge.p1);
        const p2 = this.worldToScreen(edge.p2);

        if (i === 0) ctx.moveTo(p1.x, p1.y);
        ctx.lineTo(p2.x, p2.y);
      });

      ctx.stroke();
    },

    /** @param {Rect} rect */
    drawRect(ctx, rect, color, fill = false) {
      if (!ctx) return;

      const minX = Math.min(rect.p1.x, rect.p2.x);
      const maxX = Math.max(rect.p1.x, rect.p2.x);
      const minY = Math.min(rect.p1.y, rect.p2.y);
      const maxY = Math.max(rect.p1.y, rect.p2.y);

      const p1 = this.worldToScreen({ x: minX, y: minY });
      const p2 = this.worldToScreen({ x: maxX, y: maxY });

      const w = p2.x - p1.x;
      const h = p2.y - p1.y;

      if (fill) {
        ctx.fillStyle = color + "60"; // Semi-transparent
        ctx.fillRect(p1.x, p1.y, w, h);
      }

      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      ctx.strokeRect(p1.x, p1.y, w, h);
    },

    renderWorker(workerId) {
      if (workerId >= this.contexts.length) return;

      const ctx = this.contexts[workerId];
      const canvas = this.canvases[workerId];
      if (!ctx || !canvas) return;

      const rect = this.workerRects.get(workerId);
      const isContained = this.workerContained.get(workerId);
      const area = this.workerAreas.get(workerId);

      // Check if this worker found the best rectangle (use persistent state)
      const foundBest = this.workersFoundBest.has(workerId);

      // Clear canvas with appropriate background color
      if (foundBest) {
        ctx.fillStyle = "#064e3b"; // Dark green for success
      } else {
        ctx.fillStyle = "#000000"; // Black
      }
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw polygon
      this.drawPolygon(ctx);

      // Draw worker's current rectangle
      if (rect) {
        const color = foundBest
          ? "#10b981"
          : isContained
          ? "#ffffff"
          : "#6b7280";
        this.drawRect(ctx, rect, color, true);
      }

      // Draw success indicator
      if (foundBest) {
        ctx.fillStyle = "#10b981";
        ctx.font = "bold 20px sans-serif";
        ctx.textAlign = "center";
        ctx.textBaseline = "top";
        ctx.fillText("✓ BEST", canvas.width / 2, 10);
      }
    },

    renderAll() {
      for (let i = 0; i < this.contexts.length; i++) {
        this.renderWorker(i);
      }
    },

    scheduleRender(workerId) {
      // Mark this worker as needing a render
      this.workersNeedingRender.add(workerId);

      // Schedule a render on the next animation frame if not already scheduled
      if (!this.renderScheduled) {
        this.renderScheduled = true;
        this.animationFrameId = requestAnimationFrame(() => {
          // Track FPS
          const now = performance.now();
          if (this.lastFrameTime > 0) {
            const delta = now - this.lastFrameTime;
            this.frameTimes.push(delta);

            // Keep only last 60 frame times
            if (this.frameTimes.length > 60) {
              this.frameTimes.shift();
            }

            // Calculate average FPS
            const avgDelta =
              this.frameTimes.reduce((a, b) => a + b, 0) /
              this.frameTimes.length;
            this.fps = Math.round(1000 / avgDelta);
          }
          this.lastFrameTime = now;

          // Render all workers that need updates
          this.workersNeedingRender.forEach((id) => {
            this.renderWorker(id);
          });

          // Clear the queue and mark as not scheduled
          this.workersNeedingRender.clear();
          this.renderScheduled = false;
          this.animationFrameId = null;
        });
      }
    },

    addLogEntry(msg) {
      const now = new Date();
      const timeStr = now.toLocaleTimeString("en-US", {
        hour12: false,
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        fractionalSecondDigits: 3,
      });

      let content = "";
      let type = "info";

      switch (msg.type) {
        case "init":
          content = `Initialized: ${msg.polygon.edges.length} edges, ${msg.max_cores} cores available`;
          type = "init";
          break;
        case "update":
          // Log batched update summary
          const containedCount = msg.workers.filter(
            (w) => w.is_contained
          ).length;
          const workerIds = msg.workers.map((w) => w.worker_id).join(",");
          content = `Update [${workerIds}]: ${containedCount}/${
            msg.workers.length
          } contained (Best: ${msg.current_best.toLocaleString()}, Checked: ${msg.checked_count.toLocaleString()})`;
          type = containedCount > 0 ? "success" : "update";
          break;
        case "complete":
          content = `✓ Complete! Result: ${msg.result.toLocaleString()} (Checked: ${msg.checked_count.toLocaleString()})`;
          type = "complete";
          break;
        case "status":
          const status = msg.running
            ? msg.paused
              ? "Paused"
              : "Running"
            : "Stopped";
          content = `Status: ${status}`;
          type = "status";
          break;
        default:
          content = `Unknown message type: ${msg.type}`;
          type = "error";
      }

      // Add new message at the beginning (top)
      this.messageLog.unshift({ time: timeStr, content, type });

      // Keep only the last maxLogEntries messages
      if (this.messageLog.length > this.maxLogEntries) {
        this.messageLog.pop();
      }

      // Keep scroll position at top when user is already at the top
      this.$nextTick(() => {
        const container = this.$refs.logContainer;
        if (container) {
          // Keep at top if user is within 5 pixels of the top
          const isScrolledToTop = container.scrollTop <= 5;

          if (isScrolledToTop) {
            container.scrollTop = 0;
          }
        }
      });
    },
  },
}).mount("#app");
