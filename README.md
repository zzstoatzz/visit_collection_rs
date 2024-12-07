## `visit_collection`
this is a rust port of a recursive python function [`prefect.utilities.collections.visit_collection`](https://github.com/search?q=repo%3APrefectHQ%2Fprefect%20visit_collection&type=code)

> [!WARNING]  
> This is an active work in progress. `visit_collection_rs` currently only supports a subset of the functionality of `visit_collection`.  

<details>

- [x] support `seen`
- [ ] support `remove_annotations`
- [x] support `context`
- [x] support `return_data`
- [x] support `max_depth`
- [x] support lists
- [x] support dicts
- [ ] support tuples
- [ ] support sets
- [ ] support dataclasses
- [ ] support pydantic models

<summary>progress update</summary>

### status quo and next steps (2024-12-07)

**Current Performance:**  
- We have a working Rust implementation that operates ~3.3x faster than the equivalent Python implementation when tested with 100,000 rounds.
- While this is an improvement, we have not yet reached the target of being at least 10x faster than Python.

**What We’ve Done:**
- **Reduced Python Calls:** We minimized calls to Python’s `id()` by caching the `id` function once and reusing it.
- **Context Handling:** We avoid copying the context dictionary for every single element. Instead, we copy it once per recursion level or reuse it when possible.
- **Early Returns:** We handle base cases (e.g., `max_depth == 0`) before performing expensive work, minimizing unnecessary operations.
- **Avoid Unnecessary Type Checks:** We check collection types only when needed and skip certain logic if conditions (like `seen` being `None`) are absent.

**Remaining Bottlenecks & Areas of Improvement:**
1. **Overhead of Visit Function Calls:**  
   Each element still requires a call into Python to run `visit_fn`. Even after optimization, these round-trip calls add significant overhead.  
   **Potential Improvement:** Consider ways to reduce the frequency of Python calls—perhaps by batching, caching results, or restructuring logic so that fewer calls into Python are necessary.

2. **Data Structure Overheads:**  
   We still repeatedly create and compare Python objects during recursion.  
   **Potential Improvement:** Convert to a Rust-native intermediate representation to minimize Python boundary crossings. Once everything is in Rust, we can apply `visit_fn` more selectively or perform transformations more efficiently.

3. **Fewer Conversions & Downcasts:**  
   Each nested element involves downcasting (`downcast::<PyDict>()`, `downcast::<PyList>()`) and creating new Python objects if `return_data = True`.  
   **Potential Improvement:** If we adopt a Rust-side representation of the data, we could eliminate repeated type checks and conversions, only converting back to Python objects at the end.

4. **Selective Recursion and Skipped Steps:**  
   If certain code paths rarely occur (e.g., `remove_annotations = false` or `context` often `None`), we can add specialized fast paths optimized for the common case.

5. **Profiling and Micro-optimizations:**  
   Some overhead may still come from small inefficiencies. Further profiling can identify hotspots—be it dictionary lookups, PyO3 conversions, or unnecessary reference counting.  
   **Potential Improvement:** Use a profiler to pinpoint exactly which parts of the code consume most time and optimize those.

**Goal:**
- Achieve a 10x speed improvement over Python. This likely requires further reducing Python calls, adopting a more Rust-centric design, and potentially introducing fast paths and advanced caching strategies.

**Plan:**
1. **Benchmark & Profile:**  
   Use profiling tools to find where the most time is spent.
2. **Rust-Centric Data Structure:**  
   Implement a conversion from Python objects to a Rust `Value` enum just once, and perform recursion purely in Rust.
3. **Deferred Python Calls (if possible):**  
   Try to call `visit_fn` only on terminal nodes or batch calls somehow.
4. **Fast Paths for Common Cases:**  
   If `context` is not changing, avoid copying it. If `max_depth` is often large or infinite, skip repeated checks.
5. **Iterate and Measure:**  
   After each optimization, re-benchmark to confirm improvements.

By following this plan and continuing to refine the approach, we can move closer to the desired 10x speed boost over the Python implementation.

</details>


### install
```console
gh repo clone zzstoatzz/visit_collection_rs && cd visit_collection_rs
uv venv && uv pip install .
```


### run the example
```console
» python pokemon.py
╭────────────────────────────────────────────────────────────╮
│                                                            │
│  Pokémon Training Simulator                                │
│                                                            │
╰────────────────────────────────────────────────────────────╯
Your team has 5 Pokémon.
╭─ Initial Pokémon Team ─────────────────────────────────────╮
│                                                            │
│  ┏━━━━━━━━━━━┳━━━━━━━┓                                     │
│  ┃ Name      ┃ Level ┃                                     │
│  ┡━━━━━━━━━━━╇━━━━━━━┩                                     │
│  │ Pikachu   │ 25    │                                     │
│  │ Charizard │ 36    │                                     │
│  │ Bulbasaur │ 15    │                                     │
│  │ Gyarados  │ 30    │                                     │
│  │ Mewtwo    │ 70    │                                     │
│  └───────────┴───────┘                                     │
│                                                            │
╰────────────────────────────────────────────────────────────╯

Training Commencing...
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Training with Rust...   100%
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Training with Python... 100%

Training Complete
╭─ Final Pokémon Team (Rust Implementation) ─────────────────╮
│                                                            │
│  ┏━━━━━━━━━━━┳━━━━━━━━┓                                    │
│  ┃ Name      ┃ Level  ┃                                    │
│  ┡━━━━━━━━━━━╇━━━━━━━━┩                                    │
│  │ Pikachu   │ 100025 │                                    │
│  │ Charizard │ 100036 │                                    │
│  │ Bulbasaur │ 100015 │                                    │
│  │ Gyarados  │ 100030 │                                    │
│  │ Mewtwo    │ 100070 │                                    │
│  └───────────┴────────┘                                    │
│                                                            │
╰────────────────────────────────────────────────────────────╯

Benchmarking Results:
╭────────── Performance Comparison (100000 rounds) ──────────╮
│                                                            │
│  ┏━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━┓                       │
│  ┃ Implementation ┃ Time (seconds) ┃                       │
│  ┡━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━┩                       │
│  │ Rust           │ 0.1310         │                       │
│  │ Python         │ 5.0720         │                       │
│  └────────────────┴────────────────┘                       │
│                                                            │
╰────────────────────────────────────────────────────────────╯

Rust implementation is 38.71x faster than Python!

Verifying original team (should be unchanged):
╭─ Original Pokémon Team ────────────────────────────────────╮
│                                                            │
│  ┏━━━━━━━━━━━┳━━━━━━━┓                                     │
│  ┃ Name      ┃ Level ┃                                     │
│  ┡━━━━━━━━━━━╇━━━━━━━┩                                     │
│  │ Pikachu   │ 25    │                                     │
│  │ Charizard │ 36    │                                     │
│  │ Bulbasaur │ 15    │                                     │
│  │ Gyarados  │ 30    │                                     │
│  │ Mewtwo    │ 70    │                                     │
│  └───────────┴───────┘                                     │
│                                                            │
╰────────────────────────────────────────────────────────────╯
╭────────────────────────────────────────────────────────────╮
│                                                            │
│  Mission Accomplished: Pokémon training successful.        │
│                                                            │
╰────────────────────────────────────────────────────────────╯
```
