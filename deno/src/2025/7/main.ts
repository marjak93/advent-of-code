import { getDebug, getInput } from "../../util.ts";

type NodeType = "." | "S" | "|" | "^";

class Node {
  constructor(public type: NodeType, public hasSplit: boolean = false) {}
}

class Laser {
  public path: [number, number][] = [];

  constructor(public x: number, public y: number) {}

  move(x: number, y: number) {
    this.x = x;
    this.y = y;

    this.path.push([x, y]);
  }

  isOutOfBounds(size: [number, number]) {
    return this.x < 0 || this.x >= size[0] || this.y < 0 || this.y >= size[1];
  }
}

class Grid {
  nodes: Node[][] = [];
  lasers: Laser[] = [];
  destroyedLasers: Laser[] = [];
  size: [number, number] = [0, 0];

  constructor(input: string) {
    const lines = input.trim().split("\n");

    for (const [i, line] of lines.entries()) {
      const row: Node[] = [];
      for (const [j, char] of line.split("").entries()) {
        const node = new Node(char as NodeType);

        if (char === "S") {
          this.lasers.push(new Laser(j, i));
        }

        row.push(node);
      }
      this.nodes.push(row);
    }

    this.size = [this.nodes[0].length, this.nodes.length];
  }

  get splitCount() {
    let count = 0;
    for (const row of this.nodes) {
      for (const node of row) {
        if (node.hasSplit) {
          count++;
        }
      }
    }
    return count;
  }

  hasLaserAt(x: number, y: number) {
    return this.lasers.some((laser) => laser.x === x && laser.y === y);
  }

  destroyLaser(laser: Laser) {
    this.destroyedLasers.push(laser);
    this.lasers = this.lasers.filter((l) => l.x !== laser.x || l.y !== laser.y);
  }

  tick() {
    const newLasers: Laser[] = [];

    for (const laser of this.lasers) {
      laser.move(laser.x, laser.y + 1);

      if (laser.isOutOfBounds(this.size)) {
        this.destroyLaser(laser);

        continue;
      }

      const current = this.nodes[laser.y][laser.x];

      if (current.type === ".") {
        current.type = "|";
        continue;
      }

      if (current.type === "^") {
        const l = new Laser(laser.x - 1, laser.y);
        const r = new Laser(laser.x + 1, laser.y);

        if (
          !newLasers.some((n) => n.x === l.x && n.y === l.y) &&
          !this.hasLaserAt(l.x, l.y)
        ) {
          newLasers.push(l);
          this.nodes[laser.y][laser.x - 1].type = "|";
        }

        if (
          !newLasers.some((n) => n.x === r.x && n.y === r.y) &&
          !this.hasLaserAt(r.x, r.y)
        ) {
          newLasers.push(r);
          this.nodes[laser.y][laser.x + 1].type = "|";
        }

        this.destroyLaser(laser);

        this.nodes[laser.y][laser.x].hasSplit = true;

        continue;
      }
    }

    this.lasers.push(...newLasers);
  }

  quantumTick() {
    const newLasers: Laser[] = [];

    for (const laser of this.lasers) {
      laser.move(laser.x, laser.y + 1);

      if (laser.isOutOfBounds(this.size)) {
        this.destroyLaser(laser);

        continue;
      }

      const current = this.nodes[laser.y][laser.x];

      if (current.type === ".") {
        current.type = "|";
        continue;
      }

      if (current.type === "^") {
        const l = new Laser(laser.x - 1, laser.y);
        const r = new Laser(laser.x + 1, laser.y);

        newLasers.push(l);
        this.nodes[laser.y][laser.x - 1].type = "|";

        newLasers.push(r);
        this.nodes[laser.y][laser.x + 1].type = "|";

        this.destroyLaser(laser);

        this.nodes[laser.y][laser.x].hasSplit = true;

        continue;
      }
    }

    this.lasers.push(...newLasers);
  }

  render() {
    const output: string[] = [];

    for (let i = 0; i < this.nodes.length; i++) {
      let row = `${i.toString().padStart(3, "0")} `;
      for (let j = 0; j < this.nodes[i].length; j++) {
        const node = this.nodes[i][j];
        row += node.type;
      }
      output.push(row);
    }

    return output.join("\n");
  }
}

export async function part1() {
  const input = await getInput(import.meta.url);

  const grid = new Grid(input);

  while (grid.lasers.length > 0) {
    grid.tick();
  }

  console.log(grid.render(), grid.splitCount);
}

export async function part2() {
  const input = await getDebug(import.meta.url);

  const grid = new Grid(input);

  while (grid.lasers.length > 0) {
    grid.quantumTick();
  }

  console.log(grid.render());

  console.log(grid.destroyedLasers.length);
}
