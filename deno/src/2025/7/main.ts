import { getInput } from "../../util.ts";

type Grid = Tile[][];

type Particle = {
  x: number;
  y: number;
  count: number;
};

type State = {
  GRID: Grid;
  PARTICLES: Particle[];
  SPLIT_COUNT: number;
  IGNORE_COLLISIONS: boolean;
};

enum Tile {
  Air = ".",
  Splitter = "^",
  Laser = "|",
  Emitter = "S",
}

const STATE: State = {
  GRID: [] as Grid,
  PARTICLES: [] as Particle[],
  SPLIT_COUNT: 0,
  IGNORE_COLLISIONS: true,
};

const createGrid = (input: string) => {
  const lines = input.split("\n").filter(Boolean);
  const grid: Grid = [];

  for (let y = 0; y < lines.length; y++) {
    const line = lines[y];
    grid[y] = [];

    for (let x = 0; x < line.length; x++) {
      grid[y][x] = line[x] as Tile;
    }
  }

  return grid;
};

const isOutOfBounds = (x: number, y: number) => {
  return y < 0 || y >= STATE.GRID.length || x < 0 || x >= STATE.GRID[0].length;
};

const paint = (x: number, y: number, tile: Tile) => {
  STATE.GRID[y][x] = tile;
};

const print = () => {
  for (const row of STATE.GRID) {
    console.log(row.join(""));
  }

  console.log("\n");
};

const getTile = (x: number, y: number) => {
  if (y < 0 || y >= STATE.GRID.length) {
    throw new Error(`Y coordinate ${y} is out of bounds`);
  }

  if (x < 0 || x >= STATE.GRID[y].length) {
    throw new Error(`X coordinate ${x} is out of bounds`);
  }

  return STATE.GRID[y][x];
};

const descendParticle = (particle: Particle) => {
  paint(particle.x, particle.y, Tile.Laser);
  const below = { x: particle.x, y: particle.y + 1 };

  if (isOutOfBounds(below.x, below.y)) {
    return;
  }

  const tileBelow = getTile(below.x, below.y);

  if (tileBelow !== Tile.Splitter) {
    moveParticle(particle, 0, 1);
  } else {
    STATE.SPLIT_COUNT += particle.count;

    // When ignoring collisions, check if we can merge with existing particles
    if (STATE.IGNORE_COLLISIONS) {
      // Try to merge with existing particles moving left
      const existingLeft = STATE.PARTICLES.find(
        (p) => p.x === particle.x - 1 && p.y === particle.y + 1
      );

      if (existingLeft) {
        existingLeft.count += particle.count;
      } else {
        STATE.PARTICLES.push({
          x: particle.x - 1,
          y: particle.y + 1,
          count: particle.count,
        });
      }

      // Try to merge with existing particles moving right
      const existingRight = STATE.PARTICLES.find(
        (p) => p.x === particle.x + 1 && p.y === particle.y + 1
      );

      if (existingRight) {
        existingRight.count += particle.count;
      } else {
        STATE.PARTICLES.push({
          x: particle.x + 1,
          y: particle.y + 1,
          count: particle.count,
        });
      }

      // Remove original particle
      STATE.PARTICLES.splice(STATE.PARTICLES.indexOf(particle), 1);
    } else {
      const newParticle = { x: particle.x, y: particle.y, count: 1 };
      STATE.PARTICLES.push(newParticle);
      moveParticle(particle, -1, 1);
      moveParticle(newParticle, 1, 1);
    }
  }
};

const moveParticle = (particle: Particle, dx: number, dy: number) => {
  const targetX = particle.x + dx;
  const targetY = particle.y + dy;

  if (STATE.IGNORE_COLLISIONS) {
    // When ignoring collisions, merge with existing particle at target
    const existing = STATE.PARTICLES.find(
      (p) => p.x === targetX && p.y === targetY
    );

    if (existing) {
      existing.count += particle.count;
      STATE.PARTICLES.splice(STATE.PARTICLES.indexOf(particle), 1);
      return;
    }
  } else {
    // If another particle is already at the target position, remove this particle
    const isOccupied = STATE.PARTICLES.some(
      (p) => p.x === targetX && p.y === targetY
    );

    if (isOccupied) {
      STATE.PARTICLES.splice(STATE.PARTICLES.indexOf(particle), 1);
      return;
    }
  }

  particle.x = targetX;
  particle.y = targetY;
};

export async function part1() {
  const input = await getInput(import.meta.url);
  const emitterIdx = input.indexOf(Tile.Emitter);

  if (emitterIdx === -1) {
    throw new Error("No emitter found in the top row");
  }

  STATE.IGNORE_COLLISIONS = false;

  STATE.GRID = createGrid(input);
  STATE.PARTICLES.push({ x: emitterIdx, y: 1, count: 1 });

  for (let i = 1; i < STATE.GRID.length; i++) {
    const particles = [...STATE.PARTICLES];

    for (const particle of particles) {
      descendParticle(particle);
    }
  }

  print();

  console.log(STATE.SPLIT_COUNT);
}

export async function part2() {
  const input = await getInput(import.meta.url);

  const emitterIdx = input.indexOf(Tile.Emitter);

  if (emitterIdx === -1) {
    throw new Error("No emitter found in the top row");
  }

  STATE.IGNORE_COLLISIONS = true;

  STATE.GRID = createGrid(input);
  STATE.PARTICLES = [{ x: emitterIdx, y: 1, count: 1 }];

  for (let i = 1; i < STATE.GRID.length; i++) {
    const particles = [...STATE.PARTICLES];
    for (const particle of particles) {
      descendParticle(particle);
    }
  }

  print();

  console.log(STATE.SPLIT_COUNT + 1);
}
