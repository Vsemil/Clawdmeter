"""Shared test fixtures for the daemon suites."""
from unittest.mock import AsyncMock


def connected_client() -> AsyncMock:
    """A BleakClient stand-in that is already connected.

    Both the macOS and the Windows suites drive connect_and_run this way, so
    the next bleak change to the connect path (a renamed attribute, a newly
    awaited call) is fixed here rather than in every test module.
    """
    client = AsyncMock()
    client.connect = AsyncMock(return_value=None)
    client.is_connected = True
    client.disconnect = AsyncMock()
    client.start_notify = AsyncMock()
    client.write_gatt_char = AsyncMock(return_value=None)
    return client
