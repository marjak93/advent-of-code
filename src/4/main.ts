type Direction = "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw";

class Cell {
  constructor(
    private grid: Grid,
    public position: [number, number],
    public type: "." | "@"
  ) {}

  get isRoll(): boolean {
    return this.type === "@";
  }

  get isEmpty(): boolean {
    return this.type === ".";
  }

  getNeighbor(direction: Direction): Cell {
    const [row, col] = this.position;

    let neighborPos: [number, number];

    switch (direction) {
      case "nw":
        neighborPos = [row - 1, col - 1];
        break;
      case "n":
        neighborPos = [row - 1, col];
        break;
      case "ne":
        neighborPos = [row - 1, col + 1];
        break;
      case "w":
        neighborPos = [row, col - 1];
        break;
      case "e":
        neighborPos = [row, col + 1];
        break;
      case "sw":
        neighborPos = [row + 1, col - 1];
        break;
      case "s":
        neighborPos = [row + 1, col];
        break;
      case "se":
        neighborPos = [row + 1, col + 1];
        break;
    }

    const [nRow, nCol] = neighborPos;

    return this.grid.getCell(nRow, nCol);
  }

  getNeighbors(): Cell[] {
    const directions: Direction[] = [
      "nw",
      "n",
      "ne",
      "w",
      "e",
      "sw",
      "s",
      "se",
    ];

    return directions.map((dir) => this.getNeighbor(dir));
  }

  isAccessible(): boolean {
    const neighbors = this.getNeighbors();

    const neighboringRolls = neighbors.filter((n) => n.isRoll).length;

    // console.table([
    //   [neighboringRolls],
    //   [
    //     `${this.getNeighbor("nw").type}|${this.getNeighbor("nw").position}`,
    //     `${this.getNeighbor("n").type}|${this.getNeighbor("n").position}`,
    //     `${this.getNeighbor("ne").type}|${this.getNeighbor("ne").position}`,
    //   ],
    //   [
    //     `${this.getNeighbor("w").type}|${this.getNeighbor("w").position}`,
    //     `${this.type}|${this.position}`,
    //     `${this.getNeighbor("e").type}|${this.getNeighbor("e").position}`,
    //   ],
    //   [
    //     `${this.getNeighbor("sw").type}|${this.getNeighbor("sw").position}`,

    //     `${this.getNeighbor("s").type}|${this.getNeighbor("s").position}`,
    //     `${this.getNeighbor("se").type}|${this.getNeighbor("se").position}`,
    //   ],
    // ]);

    return neighboringRolls < 4;
  }

  remove() {
    this.type = ".";
  }
}

class Grid {
  public cells: Cell[][];

  constructor(input: string) {
    const rows = input.trim().split("\n");
    this.cells = rows.map((row, rI) =>
      row.split("").map((char, cI) => {
        return new Cell(this, [rI, cI], char as "." | "@");
      })
    );
  }

  getCell(row: number, col: number): Cell {
    if (
      row < 0 ||
      col < 0 ||
      row >= this.cells.length ||
      col >= this.cells[0].length
    ) {
      return new Cell(this, [row, col], ".");
    }

    return this.cells[row][col];
  }

  isDeadlocked(): boolean {
    for (const cell of this.iter()) {
      if (cell.isAccessible()) {
        if (cell.isRoll) {
          return false;
        }
      }
    }

    return true;
  }

  *iter(): Generator<Cell> {
    for (let r = 0; r < this.cells.length; r++) {
      for (let c = 0; c < this.cells[r].length; c++) {
        yield this.cells[r][c];
      }
    }
  }
}

if (import.meta.main) {
  const input = await Deno.readTextFile(new URL("input.txt", import.meta.url));

  const grid = new Grid(input);

  // Part 1
  // let n = 0;

  // for (const cell of grid.iter()) {
  //   if (cell.isAccessible()) {
  //     if (cell.isRoll) {
  //       n++;
  //     }
  //   }
  // }

  // console.log(n);

  // Part 2
  let n = 0;

  while (!grid.isDeadlocked()) {
    for (const cell of grid.iter()) {
      if (cell.isAccessible()) {
        if (cell.isRoll) {
          cell.remove();
          n++;
        }
      }
    }
  }

  console.log(n);
}
