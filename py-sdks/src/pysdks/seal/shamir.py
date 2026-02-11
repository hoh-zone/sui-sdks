"""Shamir secret sharing over GF(2^8)."""

from __future__ import annotations

import os
from dataclasses import dataclass
from typing import Iterable, List


@dataclass
class Share:
    index: int
    share: bytes


def _gf_add(a: int, b: int) -> int:
    return a ^ b


def _gf_mul(a: int, b: int) -> int:
    p = 0
    while b:
        if b & 1:
            p ^= a
        carry = a & 0x80
        a = (a << 1) & 0xFF
        if carry:
            a ^= 0x1B
        b >>= 1
    return p


def _gf_pow(a: int, exp: int) -> int:
    result = 1
    base = a
    e = exp
    while e > 0:
        if e & 1:
            result = _gf_mul(result, base)
        base = _gf_mul(base, base)
        e >>= 1
    return result


def _gf_inv(a: int) -> int:
    if a == 0:
        raise ZeroDivisionError("cannot invert zero in GF(256)")
    return _gf_pow(a, 254)


def _eval_poly(coeffs: List[int], x: int) -> int:
    y = 0
    for coef in reversed(coeffs):
        y = _gf_mul(y, x)
        y = _gf_add(y, coef)
    return y


def split(secret: bytes, threshold: int, count: int) -> List[Share]:
    if threshold <= 0 or count <= 0 or threshold > count:
        raise ValueError("invalid threshold/count")
    shares = [bytearray(len(secret)) for _ in range(count)]
    for offset, value in enumerate(secret):
        coeffs = [value] + list(os.urandom(threshold - 1))
        for i in range(1, count + 1):
            shares[i - 1][offset] = _eval_poly(coeffs, i)
    return [Share(index=i + 1, share=bytes(data)) for i, data in enumerate(shares)]


def combine(shares: Iterable[Share]) -> bytes:
    parts = list(shares)
    if not parts:
        raise ValueError("at least one share is required")
    size = len(parts[0].share)
    if any(len(s.share) != size for s in parts):
        raise ValueError("share size mismatch")

    out = bytearray(size)
    for offset in range(size):
        value = 0
        for i, si in enumerate(parts):
            xi = si.index
            yi = si.share[offset]
            num = 1
            den = 1
            for j, sj in enumerate(parts):
                if i == j:
                    continue
                xj = sj.index
                num = _gf_mul(num, xj)
                den = _gf_mul(den, _gf_add(xi, xj))
            lagrange = _gf_mul(num, _gf_inv(den))
            value = _gf_add(value, _gf_mul(yi, lagrange))
        out[offset] = value
    return bytes(out)
