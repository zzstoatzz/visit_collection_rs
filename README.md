## `visit_collection`
this is a rust port of a recursive python function [`prefect.utilities.collections.visit_collection`](https://github.com/search?q=repo%3APrefectHQ%2Fprefect%20visit_collection&type=code)

> [!WARNING]  
> This is an active work in progress. `visit_collection_rs` currently only supports a subset of the functionality of `visit_collection`.  


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
