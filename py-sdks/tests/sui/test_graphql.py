import json
import unittest
from unittest.mock import MagicMock, patch

from pysdks.sui import GraphQLClient


class TestGraphQL(unittest.TestCase):
    def test_execute_success(self):
        c = GraphQLClient("https://example.invalid")
        mock_resp = MagicMock()
        mock_resp.__enter__.return_value.read.return_value = json.dumps({"data": {"ok": True}}).encode()
        with patch("pysdks.sui.graphql.urlopen", return_value=mock_resp):
            out = c.execute("query { ok }")
            self.assertIn("data", out)

    def test_execute_error(self):
        c = GraphQLClient("https://example.invalid")
        mock_resp = MagicMock()
        mock_resp.__enter__.return_value.read.return_value = json.dumps({"errors": [{"message": "bad"}]}).encode()
        with patch("pysdks.sui.graphql.urlopen", return_value=mock_resp):
            with self.assertRaises(RuntimeError):
                c.execute("query { bad }")


if __name__ == "__main__":
    unittest.main()
