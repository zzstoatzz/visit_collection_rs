import time
from concurrent.futures import ThreadPoolExecutor, as_completed

from prefect.utilities.collections import visit_collection as visit_collection_py
from rich.console import Console
from rich.panel import Panel
from rich.progress import BarColumn, Progress, SpinnerColumn, TextColumn
from rich.table import Table
from visit_collection_rs import visit_collection

console = Console()

TRAINING_ROUNDS = 100_000

# we support only lists and dicts (but arbitrary nesting is possible)
pokemon_team = [
    {
        "name": "Pikachu",
        "level": 25,
        "stats": {"hp": 60, "attack": 55, "defense": 40},
        "moves": ["Thunderbolt", "Quick Attack"],
    },
    {
        "name": "Charizard",
        "level": 36,
        "stats": {"hp": 78, "attack": 84, "defense": 78},
        "moves": ["Flamethrower", "Wing Attack", "Dragon Claw"],
    },
    {
        "name": "Bulbasaur",
        "level": 15,
        "stats": {"hp": 45, "attack": 49, "defense": 49},
        "moves": ["Vine Whip", "Tackle"],
    },
]


def train_pokemon(item):
    """Train the Pokémon, improving its level, stats, and moves."""
    if isinstance(item, dict):
        if "level" in item:  # This is a Pokémon
            item["level"] += 1
            if len(item["moves"]) < 4 and item["level"] % 5 == 0:
                item["moves"].append("New Move")
        elif item.keys() <= {"hp", "attack", "defense"}:  # This is a stats dict
            return {stat: value + 1 for stat, value in item.items()}
    elif isinstance(item, int):  # This could be a stat value
        return item + 1
    return item


def display_team(team, title):
    table = Table(show_header=True, header_style="bold cyan")
    table.add_column("Name", style="bold white", no_wrap=True)
    table.add_column("Level", style="bold magenta")
    table.add_column("Stats", style="bold green")
    table.add_column("Moves", style="bold yellow")
    for pokemon in team:
        stats_str = ", ".join(f"{k}: {v}" for k, v in pokemon["stats"].items())
        moves_str = ", ".join(pokemon["moves"])
        table.add_row(pokemon["name"], str(pokemon["level"]), stats_str, moves_str)
    console.print(
        Panel(
            table, title=title, title_align="left", border_style="dim", padding=(1, 2)
        )
    )


def benchmark(func, input_data, *args, progress_args=None):
    progress, task = progress_args
    start_time = time.time()
    result = input_data
    for i in range(TRAINING_ROUNDS):
        result = func(result, *args)
        if (i + 1) % (TRAINING_ROUNDS // 100) == 0:
            progress.update(task, advance=TRAINING_ROUNDS // 100)
    end_time = time.time()
    return result, end_time - start_time


if __name__ == "__main__":
    console.print(
        Panel(
            "Pokémon Advanced Training Simulator",
            style="bold white on black",
            border_style="dim",
            padding=(1, 2),
        )
    )
    console.print(
        f"Your team has [bold]{len(pokemon_team)}[/bold] Pokémon.", style="dim"
    )

    display_team(pokemon_team, "Initial Pokémon Team")

    console.print("\n[bold cyan]Training Commencing...[/bold cyan]")

    with Progress(
        SpinnerColumn(style="bold purple"),
        BarColumn(bar_width=None, style="bold cyan"),
        TextColumn("[progress.description]{task.description}", style="dim"),
        TextColumn(
            "[progress.percentage]{task.percentage:>3.0f}%", style="bold magenta"
        ),
    ) as progress:
        rust_task = progress.add_task(
            "[bold cyan]Training with Rust...", total=TRAINING_ROUNDS
        )
        python_task = progress.add_task(
            "[bold cyan]Training with Python...", total=TRAINING_ROUNDS
        )

        with ThreadPoolExecutor() as executor:
            futures = {
                executor.submit(
                    benchmark,
                    visit_collection,
                    pokemon_team,
                    train_pokemon,
                    True,
                    progress_args=(progress, rust_task),
                ): "Rust",
                executor.submit(
                    benchmark,
                    visit_collection_py,
                    pokemon_team,
                    train_pokemon,
                    True,
                    progress_args=(progress, python_task),
                ): "Python",
            }

            results = {}
            for future in as_completed(futures):
                impl = futures[future]
                result, duration = future.result()
                results[impl] = (result, duration)

    rs_result, rs_time = results["Rust"]
    py_result, py_time = results["Python"]

    assert (
        rs_result == py_result
    ), f"Rust implementation doesn't match Python. Python: {py_result}, Rust: {rs_result}"

    console.print("\n[bold green]Training Complete[/bold green]")

    display_team(rs_result, "Final Pokémon Team (Rust Implementation)")

    console.print("\n[bold]Benchmarking Results:[/bold]", style="dim")
    benchmark_table = Table(show_header=True, header_style="bold cyan")
    benchmark_table.add_column("Implementation", style="bold white")
    benchmark_table.add_column("Time (seconds)", style="bold magenta")
    benchmark_table.add_row("Rust", f"{rs_time:.4f}")
    benchmark_table.add_row("Python", f"{py_time:.4f}")
    console.print(
        Panel(
            benchmark_table,
            title=f"Performance Comparison ({TRAINING_ROUNDS} rounds)",
            border_style="dim",
            padding=(1, 2),
        )
    )

    speedup = py_time / rs_time
    console.print(
        f"\nRust implementation is [bold]{speedup:.2f}x faster[/bold] than Python!",
        style="bold purple",
    )

    # Verify that the original team wasn't modified
    console.print(
        "\n[bold]Verifying original team (should be unchanged):[/bold]", style="dim"
    )
    display_team(pokemon_team, "Original Pokémon Team")

    console.print(
        Panel(
            "Mission Accomplished: Pokémon training successful.",
            style="bold green on black",
            border_style="bold",
            padding=(1, 2),
        )
    )
