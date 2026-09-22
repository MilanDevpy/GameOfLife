# Yo, so I still need to finish the ReadME but tbh you can just play with parameters in src/config.rs ;)

## GameOfLife
You can modify the consts to change the parameters. Here are some variants of Conway's GOF:

**Conway GOF:**
*The original GOF*
```rust
const CELL_TYPE: CellType = CellType {
    b: &[3],
    s: &[2,3],
    color: WHITE,
};
```
**Coagulations:**
*It stabilizes with a majority of living cells, pretty cool*
```rust
const CELL_TYPE: CellType = CellType {
    b: &[3, 7, 8],
    s: &[2, 3, 5, 6, 7, 8],
    color: YELLOW,
};
```

**1/1**
*Very impressive for only one parameter, but try it in Full Circle*

```rust
const CELL_TYPE: CellType = CellType {
    b: &[1],
    s: &[1],
    color: RED,
};
```
**Maze**
*One of my favorites*
```rust
const CELL_TYPE: CellType = CellType {
    b: &[3],
    s: &[1, 2, 3, 4, 5],
    color: BLUE,
};
```
