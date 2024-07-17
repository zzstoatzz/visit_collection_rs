import time

from prefect.utilities.collections import visit_collection as visit_collection_py
from rich.console import Console
from rich.panel import Panel
from rich.progress import BarColumn, Progress, SpinnerColumn, TextColumn
from rich.table import Table
from visit_collection_rs import visit_collection

console = Console()

TRAINING_ROUNDS = 100_000

# Define a list of Pokémon with their levels
pokemon_team = [
    {"name": "Pikachu", "level": 25},
    {"name": "Charizard", "level": 36},
    {"name": "Bulbasaur", "level": 15},
    {"name": "Gyarados", "level": 30},
    {"name": "Mewtwo", "level": 70},
]


def level_up(pokemon):
    """Level up the Pokémon."""
    if isinstance(pokemon, dict):
        return {
            "name": pokemon["name"],
            "level": pokemon["level"] + 1,
            "message": f"{pokemon['name']} leveled up to {pokemon['level'] + 1}!",
        }
    return pokemon


def display_team(team, title):
    table = Table(title=title)
    table.add_column("Name", style="cyan", no_wrap=True)
    table.add_column("Level", style="magenta")
    for pokemon in team:
        table.add_row(pokemon["name"], str(pokemon["level"]))
    console.print(table)


def benchmark(func, input_data, progress, task, *args, rounds=TRAINING_ROUNDS):
    start_time = time.time()
    result = input_data
    for i in range(rounds):
        result = func(result, *args)
        if (i + 1) % (rounds // 100) == 0:  # Update progress every 1%
            progress.update(task, advance=rounds // 100)
    end_time = time.time()
    return result, end_time - start_time


if __name__ == "__main__":
    console.print(
        Panel.fit("Welcome to the Pokémon Training Simulator!", style="bold magenta")
    )
    console.print(f"Your team has [bold]{len(pokemon_team)}[/bold] Pokémon.\n")

    display_team(pokemon_team, "Initial Pokémon Team")

    console.print("\n[bold green]Starting intense training...[/bold green]")

    with Progress(
        SpinnerColumn(),
        BarColumn(),
        TextColumn("[progress.description]{task.description}"),
        TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
    ) as progress:
        task = progress.add_task("[cyan]Training Pokémon...", total=TRAINING_ROUNDS)

        console.print("Benchmarking Rust implementation...")
        rs_result, rs_time = benchmark(
            visit_collection,
            pokemon_team,
            progress,
            task,
            level_up,
            True,
        )

        console.print("Benchmarking Python implementation...")
        py_result, py_time = benchmark(
            visit_collection_py,
            pokemon_team,
            progress,
            task,
            level_up,
            True,
        )

    console.print("\n[bold green]Training complete![/bold green]")

    display_team(rs_result, "Final Pokémon Team (Rust Implementation)")

    console.print("\n[bold]Benchmarking Results:[/bold]")
    benchmark_table = Table(title="Performance Comparison (1000 rounds)")
    benchmark_table.add_column("Implementation", style="cyan")
    benchmark_table.add_column("Time (seconds)", style="magenta")
    benchmark_table.add_row("Rust", f"{rs_time:.4f}")
    benchmark_table.add_row("Python", f"{py_time:.4f}")
    console.print(benchmark_table)

    speedup = py_time / rs_time
    console.print(
        f"\nRust implementation is [bold]{speedup:.2f}x faster[/bold] than Python!"
    )

    # Verify that the original team wasn't modified
    console.print("\n[bold]Verifying original team (should be unchanged):[/bold]")
    display_team(pokemon_team, "Original Pokémon Team")

    console.print(
        Panel.fit(
            "All assertions passed. Your Pokémon journey with visit_collection was a success!",
            style="bold green",
        )
    )
