"""Shared batch helpers for sync/async Sui client methods."""

from __future__ import annotations

from typing import Awaitable, Callable, Iterable, List, TypeVar

T = TypeVar("T")
R = TypeVar("R")


def map_sync(items: Iterable[T], fn: Callable[[T], R]) -> List[R]:
    return [fn(item) for item in items]


async def map_async(items: Iterable[T], fn: Callable[[T], Awaitable[R]]) -> List[R]:
    results: List[R] = []
    for item in items:
        results.append(await fn(item))
    return results
