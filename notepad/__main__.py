import logging
import sys

import anyio
import platformdirs
from rich.console import Console


NOTEPAD_EXE_APPNAME = "Microsoft.WindowsNotepad_8wekyb3d8bbwe"


class CLI:
    def __init__(self):
        self.console = Console()
        self.notepad_data_path = platformdirs.user_data_path(appauthor='Packages', appname=NOTEPAD_EXE_APPNAME, roaming=False)
        self.notepad_state_path = anyio.Path(self.notepad_data_path / "LocalState" / "TabState")

    async def read_all(self):
        """
        Read all Windows 11 notepad.exe .bin state file
        """

        if not await self.notepad_state_path.exists():
            self.console.print(f"[red]Notepad TabState directory not found at {self.notepad_state_path}[/red]")
            return

        async for path in self.notepad_state_path.glob("*.bin"):
            if path.name.endswith((".0.bin", ".1.bin", ".2.bin", ".3.bin", ".4.bin", ".5.bin", ".6.bin", ".7.bin", ".8.bin", ".9.bin")):
                continue
            await self.read(str(path))

    def _normalize_line_endings(self, text):
        return '\n'.join(text.splitlines())

    async def read(self, filename):
        """
        Read one Windows 11 notepad.exe .bin state file
        """
        path = anyio.Path(filename)
        if not await path.exists():
            self.console.print(f"[red]File not found: {filename}[/red]")
            return

        state = self._decode_bin(filename)
        
        self.console.print(f"[bold cyan]File:[/bold cyan] {filename}")
        if state:
            from rich.table import Table
            table = Table(show_header=False, box=None)
            table.add_row("[bold]Original Path:[/bold]", str(state.path))
            table.add_row("[bold]Is Saved:[/bold]", str(state.is_saved_file))
            table.add_row("[bold]Encoding:[/bold]", str(state.encoding))
            table.add_row("[bold]CR Type:[/bold]", str(state.cr_type))
            table.add_row("[bold]Cursor:[/bold]", f"{state.cursor_start} - {state.cursor_end}")
            table.add_row("[bold]Word Wrap:[/bold]", str(state.word_wrap))
            table.add_row("[bold]RTL:[/bold]", str(state.rtl))
            
            self.console.print(table)
            
            if state.file_content:
                self.console.print("[bold green]Content:[/bold green]")
                self.console.print(self._normalize_line_endings(state.file_content))
            
            if state.unsaved_chunks_str:
                self.console.print("\n[bold red]--- Unsaved Changes ---[/bold red]")
                self.console.print(state.unsaved_chunks_str)
        else:
            self.console.print("[yellow]No readable state found.[/yellow]")
        self.console.print("-" * 40)

    def _decode_bin(self, filename):
        """
        Decode binary data from Notepad .bin file using Rust-backed parser.
        """
        try:
            from notepad import _core
            return _core.parse_bin(filename)
        except ImportError as e:
            self.console.print(f"[red]Error: Rust core module not found. Please build the project using maturin. ({e})[/red]")
            sys.exit(1)
        except Exception as e:
            self.console.print(f"[red]Error parsing with Rust: {e}[/red]")
            sys.exit(2)


def main():
    import fire
    from rich.logging import RichHandler

    logging.basicConfig(level=logging.DEBUG, handlers=[RichHandler()])
    fire.Fire(CLI)


if __name__ == '__main__':
    main()
