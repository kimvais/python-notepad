import anyio
import logging

import platformdirs

from .exceptions import TabStateNotFoundError, FileNotFoundError, DecodeError

NOTEPAD_EXE_APPNAME = "Microsoft.WindowsNotepad_8wekyb3d8bbwe"
logger = logging.getLogger(__name__)


def normalize_line_endings(text: str) -> str:
    return "\n".join(text.splitlines())


def decode_bin(filename: str):
    """
    Decode binary data from Notepad .bin file using Rust-backed parser.
    """
    try:
        from notepad import _core

        return _core.parse_bin(filename)
    except ImportError as e:
        logger.fatal(
            f"Rust core module not found. Please build the project using maturin. ({e})"
        )
        raise
    except Exception as e:
        raise DecodeError(f"Error parsing with Rust: {e}") from e


class NotepadState:
    def __init__(self):
        self.notepad_data_path = platformdirs.user_data_path(
            appauthor="Packages", appname=NOTEPAD_EXE_APPNAME, roaming=False
        )
        self.notepad_state_path = anyio.Path(
            self.notepad_data_path / "LocalState" / "TabState"
        )

    async def get_tab_state_files(self):
        if not await self.notepad_state_path.exists():
            raise TabStateNotFoundError(
                f"Notepad TabState directory not found at {self.notepad_state_path}"
            )

        files = []
        async for path in self.notepad_state_path.glob("*.bin"):
            if path.name.endswith(
                (
                    ".0.bin",
                    ".1.bin",
                    ".2.bin",
                    ".3.bin",
                    ".4.bin",
                    ".5.bin",
                    ".6.bin",
                    ".7.bin",
                    ".8.bin",
                    ".9.bin",
                )
            ):
                continue
            files.append(str(path))
        return files

    async def get_state(self, filename: str):
        path = anyio.Path(filename)
        if not await path.exists():
            raise FileNotFoundError(f"File not found: {filename}")

        return decode_bin(filename)
