# Rust Learning Roadmap: From Advent of Code to AI

Three stages: 40 Advent of Code puzzles ordered by concept, a model-management app, then an AI capstone.

For each puzzle, solve both parts, then compare with others' Rust solutions. That comparison teaches idiomatic Rust faster than anything else.

---

## Stage 1: 40 Advent of Code Puzzles

**Setup:** use one Cargo workspace, put each puzzle's example input in a `#[test]`, and run `cargo clippy` on every solution. Clippy is a great Rust teacher.

### Phase 1: Basics (iterators, strings, Vec, parsing)

- [X] 1. **2024 D1 Historian Hysteria**: `sort`, `zip`, `iter().sum()`
- [X] 2. **2022 D1 Calorie Counting**: `split`, `map`, `collect`
- [X] 3. **2024 D2 Red-Nosed Reports**: `windows()`, `all`/`any`
- [X] 4. **2022 D6 Tuning Trouble**: slices and `windows()`
- [X] 5. **2023 D1 Trebuchet?!**: `&str` vs `String`, `find` returning `Option`
- [X] 6. **2022 D3 Rucksack Reorganization**: `HashSet`, `chars` vs `bytes`
- [ ] 7. **2022 D4 Camp Cleanup**: implement `FromStr`, return `Result`
- [ ] 8. **2023 D2 Cube Conundrum**: structs and parsing into them

### Phase 2: Enums, match, Option

- [ ] 9. **2022 D2 Rock Paper Scissors**: enums with `impl` methods
- [ ] 10. **2022 D10 Cathode-Ray Tube**: enum instructions, a simple VM
- [ ] 11. **2023 D7 Camel Cards**: `derive(PartialOrd, Ord)` on enums, custom sorting
- [ ] 12. **2024 D7 Bridge Repair**: an operator enum and recursion
- [ ] 13. **2023 D15 Lens Library**: an enum carrying data (`Remove`, `Insert(u8)`)
- [ ] 14. **2024 D3 Mull It Over**: a hand-written parser (then try the `regex` crate)
- [ ] 15. **2023 D3 Gear Ratios**: `checked_sub` and `Option` for grid bounds
- [ ] 16. **2024 D9 Disk Fragmenter**: `Vec<Option<usize>>`, in-place mutation

### Phase 3: Collections

- [ ] 17. **2022 D9 Rope Bridge**: `HashSet<(i32, i32)>`, `Copy` structs
- [ ] 18. **2023 D4 Scratchcards**: `HashSet` intersection, counting
- [ ] 19. **2024 D5 Print Queue**: `sort_by` with a custom comparator
- [ ] 20. **2022 D5 Supply Stacks**: `Vec<Vec<char>>` as stacks, messy input
- [ ] 21. **2022 D12 Hill Climbing Algorithm**: BFS with `VecDeque`
- [ ] 22. **2023 D17 Clumsy Crucible**: Dijkstra with `BinaryHeap` and `Reverse`
- [ ] 23. **2024 D11 Plutonian Pebbles**: `HashMap` counting, `entry()` API
- [ ] 24. **2023 D8 Haunted Wasteland**: `HashMap<&str, (&str, &str)>` (your first lifetimes)

### Phase 4: Grids and Simulation

- [ ] 25. **2022 D8 Treetop Tree House**: a 2D grid struct with index helpers
- [ ] 26. **2022 D14 Regolith Reservoir**: simulation, sparse vs dense grids
- [ ] 27. **2024 D6 Guard Gallivant**: a `Direction` enum with a `turn_right()` method
- [ ] 28. **2023 D10 Pipe Maze**: a tile enum, neighbor logic
- [ ] 29. **2024 D15 Warehouse Woes**: a cell enum, mutable grid updates

### Phase 5: Data-Structure Modeling (the most important phase)

- [ ] 30. **2022 D7 No Space Left On Device**: a file tree. Try `Rc<RefCell<>>` first, then redo it with an arena (`Vec` plus indices). This one teaches ownership.
- [ ] 31. **2022 D11 Monkey in the Middle**: modeling with structs, an operation enum vs closures
- [ ] 32. **2022 D13 Distress Signal**: a recursive enum (`List(Vec<Packet>)`, `Int`), implementing `Ord` by hand
- [ ] 33. **2022 D21 Monkey Math**: an expression tree as a recursive enum
- [ ] 34. **2023 D19 Aplenty**: modeling rules and workflows, range splitting
- [ ] 35. **2023 D20 Pulse Propagation**: trait objects vs enums for modules (try both)
- [ ] 36. **2024 D24 Crossed Wires**: a logic-gate graph, topological evaluation
- [ ] 37. **2023 D22 Sand Slabs**: a dependency graph between structs

### Phase 6: Algorithms and Performance

- [ ] 38. **2023 D5 If You Give A Seed A Fertilizer**: interval arithmetic
- [ ] 39. **2023 D12 Hot Springs**: memoized recursion with a `HashMap` cache
- [ ] 40. **2022 D16 Proboscidea Volcanium**: bitmask DP (hard, so save it for last)

---

## Stage 2: Model-Management App (`modelctl`)

A CLI plus a small library that combines everything from Stage 1.

- [ ] 1. **Model registry**: a `Provider` enum, a `ModelSpec` struct with capabilities, loaded from a TOML config with serde.
- [ ] 2. **Inference trait**: `trait Inference { async fn complete(...); async fn stream(...); }`, with one implementation against a real API using reqwest and tokio.
- [ ] 3. **Streaming**: parse SSE into a typed `StreamEvent` enum.
- [ ] 4. **Codex event parsing**: run `codex exec --json` and parse each JSONL line into a typed enum. Same skill as step 3, applied to a subprocess.
- [ ] 5. **Routing and fallback**: pick a model by task type, retry with another on failure, with proper error types (`thiserror`).
- [ ] 6. **Usage tracking**: tokens, latency, and cost per request, stored in a `HashMap` or SQLite.

---

## Stage 3: AI Capstone

Turn `modelctl` into a real SDK for your internal GenAI platform.

- [ ] 1. **Model inference and model management** modules
- [ ] 2. **Agent runtime** module
- [ ] 3. **A2A exposure** using the `ra2a` crate

By then every piece will be familiar, since Stage 2 has already built most of the foundations.