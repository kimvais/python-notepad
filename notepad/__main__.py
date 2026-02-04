import logging

from rich.console import Console

from .logic import NotepadState, normalize_line_endings
from .exceptions import NotepadError


class CLI:
    def __init__(self):
        self.console = Console()
        self.state_manager = NotepadState()

    async def read_all(self):
        """
        Read all Windows 11 notepad.exe .bin state file
        """
        try:
            files = await self.state_manager.get_tab_state_files()
            for file in files:
                await self.read(file)
        except NotepadError as e:
            self.console.print(f"[red]{e}[/red]")

    async def read(self, filename):
        """
        Read one Windows 11 notepad.exe .bin state file
        """
        try:
            state = await self.state_manager.get_state(filename)

            self.console.print(f"[bold cyan]File:[/bold cyan] {filename}")
            if state:
                from rich.table import Table

                table = Table(show_header=False, box=None)
                table.add_row("[bold]Original Path:[/bold]", str(state.path))
                table.add_row("[bold]Is Saved:[/bold]", str(state.is_saved_file))
                table.add_row(
                    "[bold]Encoding:[/bold]",
                    state.encoding.name if state.encoding else "None",
                )
                table.add_row(
                    "[bold]CR Type:[/bold]",
                    state.cr_type.name if state.cr_type else "None",
                )
                table.add_row(
                    "[bold]Cursor:[/bold]", f"{state.cursor_start} - {state.cursor_end}"
                )
                table.add_row("[bold]Word Wrap:[/bold]", str(state.word_wrap))
                table.add_row("[bold]RTL:[/bold]", str(state.rtl))

                self.console.print(table)

                if state.file_content:
                    self.console.print("[bold green]Content:[/bold green]")
                    self.console.print(normalize_line_endings(state.file_content))

                if state.unsaved_chunks_str:
                    self.console.print("\n[bold red]--- Unsaved Changes ---[/bold red]")
                    self.console.print(state.unsaved_chunks_str)
            else:
                self.console.print("[yellow]No readable state found.[/yellow]")
            self.console.print("-" * 40)
        except ImportError as e:
            self.console.print(f"[red]{e}[/red]")
        except NotepadError as e:
            self.console.print(f"[red]{e}[/red]")
        except Exception as e:
            self.console.print(f"[red]Unexpected error: {e}[/red]")


def main():
    import fire
    from rich.logging import RichHandler

    logging.basicConfig(level=logging.DEBUG, handlers=[RichHandler()])
    fire.Fire(CLI)


if __name__ == "__main__":
    main()
