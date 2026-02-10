import unittest

from pysdks.sui.transactions import AsyncResolver, ResolverPluginError, Transaction


class TestAsyncResolver(unittest.IsolatedAsyncioTestCase):
    async def test_async_resolver_plugin_pipeline(self):
        tx = Transaction()
        tx.object("0x123")
        tx.object("0x456")

        seen = {"count": 0, "ordered": []}

        async def plugin_one(ctx):
            seen["count"] = len(ctx.unresolved_inputs)
            seen["ordered"].append("one")

        async def plugin_two(ctx):
            seen["ordered"].append("two")
            # Resolver should pass the same context through all plugins.
            ctx.transaction.set_sender_if_not_set("0xabc")

        resolver = AsyncResolver()
        resolver.add_plugin(plugin_one)
        resolver.add_plugin(plugin_two)

        context = await resolver.resolve(tx)
        self.assertEqual(2, seen["count"])
        self.assertEqual(["one", "two"], seen["ordered"])
        self.assertEqual("0xabc", context.transaction.data.sender)

    async def test_async_resolver_accepts_sync_and_async_plugins(self):
        tx = Transaction()
        tx.object("0x777")
        seen = []

        def sync_plugin(ctx):
            seen.append(("sync", len(ctx.unresolved_inputs)))

        async def async_plugin(ctx):
            seen.append(("async", len(ctx.unresolved_inputs)))

        resolver = AsyncResolver()
        resolver.add_plugin(sync_plugin)
        resolver.add_plugin(async_plugin)

        context = await resolver.resolve(tx)
        self.assertEqual(1, len(context.unresolved_inputs))
        self.assertEqual([("sync", 1), ("async", 1)], seen)

    async def test_async_resolver_wraps_plugin_error(self):
        tx = Transaction()
        tx.object("0x999")

        async def bad_plugin(_ctx):
            raise ValueError("boom")

        resolver = AsyncResolver()
        resolver.add_plugin(bad_plugin)

        with self.assertRaises(ResolverPluginError) as ctx:
            await resolver.resolve(tx)

        self.assertEqual(0, ctx.exception.index)
        self.assertEqual("bad_plugin", ctx.exception.plugin_name)
        self.assertIn("boom", str(ctx.exception))
        payload = ctx.exception.to_dict()
        self.assertEqual("ResolverPluginError", payload["error_type"])
        self.assertEqual(0, payload["index"])
        self.assertEqual("bad_plugin", payload["plugin_name"])
        self.assertEqual("ValueError", payload["cause_type"])
        self.assertEqual("boom", payload["cause_message"])
        self.assertIn("ResolverPluginError(", repr(ctx.exception))


if __name__ == "__main__":
    unittest.main()
