class NotepadError(Exception):
    """Base exception for notepad errors."""


class TabStateNotFoundError(NotepadError):
    """Raised when the Notepad TabState directory is not found."""


class FileNotFoundError(NotepadError, FileNotFoundError):
    """Raised when a specific .bin file is not found."""


class DecodeError(NotepadError):
    """Raised when there is an error decoding the .bin file."""
