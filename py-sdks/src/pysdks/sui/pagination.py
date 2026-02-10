"""Shared pagination helpers for sync/async Sui client iterators."""

from __future__ import annotations

from typing import Any, AsyncIterator, Awaitable, Callable, Dict, Iterator, Optional


def iter_paginated_items(
    fetch_page: Callable[[Optional[str]], Dict[str, Any]],
    *,
    start_cursor: Optional[str] = None,
    max_items: Optional[int] = None,
) -> Iterator[Dict[str, Any]]:
    cursor = start_cursor
    emitted = 0
    while True:
        page = fetch_page(cursor)
        for item in page.get("data", []):
            yield item
            emitted += 1
            if max_items is not None and emitted >= max_items:
                return
        if not page.get("hasNextPage"):
            return
        next_cursor = page.get("nextCursor")
        if next_cursor is None or next_cursor == cursor:
            return
        cursor = next_cursor


async def aiter_paginated_items(
    fetch_page: Callable[[Optional[str]], Awaitable[Dict[str, Any]]],
    *,
    start_cursor: Optional[str] = None,
    max_items: Optional[int] = None,
) -> AsyncIterator[Dict[str, Any]]:
    cursor = start_cursor
    emitted = 0
    while True:
        page = await fetch_page(cursor)
        for item in page.get("data", []):
            yield item
            emitted += 1
            if max_items is not None and emitted >= max_items:
                return
        if not page.get("hasNextPage"):
            return
        next_cursor = page.get("nextCursor")
        if next_cursor is None or next_cursor == cursor:
            return
        cursor = next_cursor
