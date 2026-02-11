"""GraphQL query builder for Python SDK."""

from typing import Dict, Any, Optional, Callable


class GraphQLQueryBuilder:
    """Builder for constructing GraphQL queries."""
    
    def __init__(self, client, cache, plugins: list):
        self.client = client
        self.cache = cache
        self.plugins = plugins or []
    
    def build(self, query: str, variables: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Build query options."""
        return {
            "query": query,
            "variables": variables or {},
        }
    
    def execute(self, query: str, variables: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Execute a GraphQL query."""
        options = self.build(query, variables)
        
        cache_key = self._cache_key(query, variables or {})
        
        if self.cache and cache_key:
            result = self.cache.get(cache_key)
            if result and result.get("data") is not None:
                return result["data"]
        
        return self._execute_query(options)
    
    def _cache_key(self, query: str, variables: Dict[str, Any]) -> str:
        """Generate a cache key."""
        import json
        return query + json.dumps(variables, sort_keys=True)


class NamedQueries:
    """Registry for named queries."""
    
    def __init__(self):
        self.queries: Dict[str, Dict[str, Any]] = {}
    
    def register(self, name: str, query: str, result_schema: Dict) -> None:
        """Register a named query."""
        self.queries[name] = {
            "name": name,
            "query": query,
            "result_schema": result_schema,
        }
    
    def get(self, name: str) -> Optional[Dict]:
        """Get a named query."""
        return self.queries.get(name)
    
    def list(self) -> list[str]:
        """List all query names."""
        return list(self.queries.keys())


class QueryCache:
    """Cache for GraphQL query results."""
    
    def __init__(self):
        self.cache: Dict[str, Dict[str, Any]] = {}
    
    def get(self, key: str) -> Optional[Dict]:
        """Get from cache."""
        return self.cache.get(key)
    
    def set(self, key: str, result: Dict) -> None:
        """Set in cache."""
        self.cache[key] = result
    
    def invalidate(self, key: str) -> None:
        """Invalidate a cache key."""
        self.cache.pop(key, None)


def new_query_builder(client, cache=None, plugins=None) -> GraphQLQueryBuilder:
    """Create a new query builder."""
    return GraphQLQueryBuilder(client, cache, plugins)