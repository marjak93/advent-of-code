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
      /** @type {Rect | null} */
      currentRect: null,
      /** @type {number} */
      checkedCount: 0,
      /** @type {number} */
      currentArea: 0,
      /** @type {number} */
      bestArea: 0,
      /** @type {boolean} */
      isContained: false,
      /** @type {boolean} */
      isRunning: false,
      /** @type {boolean} */
      isPaused: false,
      /** @type {number} */
      speed: 200, // milliseconds delay (20-1000ms range)
      /** @type {string} */
      connectionStatus: "Connecting...",
      /** @type {string} */
      statusMessage: "Ready to start",
      /** @type {number} */
      reconnectAttempts: 0,
      /** @type {number} */
      maxReconnectAttempts: 5,

      // Canvas settings
      /** @type {HTMLCanvasElement | null} */
      canvas: null,
      /** @type {CanvasRenderingContext2D | null} */
      ctx: null,
      /** @type {number} */
      scale: 1,
      /** @type {number} */
      offsetX: 0,
      /** @type {number} */
      offsetY: 0,
      /** @type {number} */
      padding: 50,
    };
  },
  computed: {
    connectionClass() {
      if (this.connectionStatus === "Connected") return "connected";
      if (this.connectionStatus === "Disconnected") return "disconnected";
      return "connecting";
    },
    currentStatusClass() {
      return this.isContained ? "contained" : "rejected";
    },
    currentStatus() {
      if (!this.currentRect) return "-";
      return this.isContained ? "✓ CONTAINED" : "✗ Rejected";
    },
    statusClass() {
      if (this.isRunning && !this.isPaused) return "status-running";
      if (this.isPaused) return "status-paused";
      if (this.bestArea > 0 && !this.isRunning) return "status-complete";
      return "status-idle";
    },
    speedLabel() {
      const delayMs = this.speed;
      if (delayMs < 100) return "Very Fast";
      if (delayMs < 300) return "Fast";
      if (delayMs < 600) return "Medium";
      if (delayMs < 850) return "Slow";
      return "Very Slow";
    },
    delayMs() {
      return this.speed.toFixed(0);
    },
  },
  mounted() {
    this.canvas = this.$refs.canvas;
    if (this.canvas) {
      this.ctx = this.canvas.getContext("2d");
    }
    this.connectWebSocket();
    this.setupResponsiveCanvas();
    window.addEventListener("resize", this.handleResize);
  },
  beforeUnmount() {
    if (this.ws) {
      this.ws.close();
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

      switch (msg.type) {
        case "init":
          this.polygon = msg.polygon;
          console.log(
            "[MSG] Polygon initialized with",
            msg.polygon.edges.length,
            "edges"
          );
          this.setupViewport();
          this.render();
          break;

        case "update":
          this.currentRect = msg.rect;
          this.currentArea = msg.area;
          this.isContained = msg.is_contained;
          this.bestArea = msg.current_best;
          this.checkedCount = msg.checked_count;
          this.render();
          break;

        case "complete":
          this.bestArea = msg.result;
          this.checkedCount = msg.checked_count;
          this.isRunning = false;
          this.isPaused = false;
          this.currentRect = null;
          this.statusMessage = `Complete! Final answer: ${msg.result.toLocaleString()}`;
          console.log("[MSG] Algorithm complete:", msg.result);
          this.render();
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

      // Reset state
      this.checkedCount = 0;
      this.currentArea = 0;
      this.bestArea = 0;
      this.currentRect = null;
      this.render();

      // Convert speed from milliseconds to microseconds
      const speedMicros = this.speed * 1000;
      console.log(
        "[UI] Starting with speed:",
        this.speed,
        "ms (",
        speedMicros,
        "microseconds)"
      );
      this.sendMessage({ type: "start", speed: speedMicros });
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
      this.currentRect = null;
      this.render();
    },

    setSpeed() {
      // Convert speed from milliseconds to microseconds
      const speedMicros = this.speed * 1000;
      console.log(
        "[UI] Setting speed to:",
        this.speed,
        "ms (",
        speedMicros,
        "microseconds)"
      );
      if (this.isRunning) {
        this.sendMessage({ type: "set_speed", speed: speedMicros });
      }
    },

    setupResponsiveCanvas() {
      if (!this.canvas) return;

      const container = this.canvas.parentElement;
      if (!container) return;

      const maxWidth = container.clientWidth - 40; // Account for padding
      const maxHeight = Math.min(window.innerHeight * 0.7, 900);

      // Maintain aspect ratio
      const aspectRatio = 4 / 3;
      let width = maxWidth;
      let height = width / aspectRatio;

      if (height > maxHeight) {
        height = maxHeight;
        width = height * aspectRatio;
      }

      this.canvas.width = width;
      this.canvas.height = height;

      console.log("[CANVAS] Resized to:", width, "x", height);

      if (this.polygon) {
        this.setupViewport();
        this.render();
      }
    },

    handleResize() {
      this.setupResponsiveCanvas();
    },

    setupViewport() {
      if (!this.polygon || !this.canvas) return;

      const [minX, maxX, minY, maxY] = this.polygon.bounding_box;
      const polyWidth = maxX - minX;
      const polyHeight = maxY - minY;

      const scaleX = (this.canvas.width - 2 * this.padding) / polyWidth;
      const scaleY = (this.canvas.height - 2 * this.padding) / polyHeight;
      this.scale = Math.min(scaleX, scaleY);

      const scaledWidth = polyWidth * this.scale;
      const scaledHeight = polyHeight * this.scale;
      this.offsetX = (this.canvas.width - scaledWidth) / 2 - minX * this.scale;
      this.offsetY =
        (this.canvas.height - scaledHeight) / 2 - minY * this.scale;

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

    drawPolygon() {
      if (!this.polygon || !this.ctx) return;

      this.ctx.strokeStyle = "#ffffff";
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();

      this.polygon.edges.forEach((edge, i) => {
        const p1 = this.worldToScreen(edge.p1);
        const p2 = this.worldToScreen(edge.p2);

        if (i === 0) this.ctx.moveTo(p1.x, p1.y);
        this.ctx.lineTo(p2.x, p2.y);
      });

      this.ctx.stroke();
    },

    /** @param {Rect} rect */
    drawRect(rect, color, fill = false) {
      if (!this.ctx) return;

      const minX = Math.min(rect.p1.x, rect.p2.x);
      const maxX = Math.max(rect.p1.x, rect.p2.x);
      const minY = Math.min(rect.p1.y, rect.p2.y);
      const maxY = Math.max(rect.p1.y, rect.p2.y);

      const p1 = this.worldToScreen({ x: minX, y: minY });
      const p2 = this.worldToScreen({ x: maxX, y: maxY });

      const w = p2.x - p1.x;
      const h = p2.y - p1.y;

      if (fill) {
        this.ctx.fillStyle = color + "60"; // Semi-transparent
        this.ctx.fillRect(p1.x, p1.y, w, h);
      }

      this.ctx.strokeStyle = color;
      this.ctx.lineWidth = 2;
      this.ctx.strokeRect(p1.x, p1.y, w, h);
    },

    render() {
      if (!this.ctx || !this.canvas) return;

      // Clear canvas with black background
      this.ctx.fillStyle = "#000000";
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

      // Draw polygon
      this.drawPolygon();

      // Draw current rectangle - use white for contained, light gray for rejected
      if (this.currentRect) {
        const color = this.isContained ? "#ffffff" : "#6b7280";
        this.drawRect(this.currentRect, color, true);
      }
    },
  },
}).mount("#app");
