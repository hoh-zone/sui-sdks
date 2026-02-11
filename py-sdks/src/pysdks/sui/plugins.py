"""Transaction plugin system for Python SDK."""

from typing import Protocol, TYPE_CHECKING
from abc import ABC, abstractmethod


class TransactionKind:
    MOVE_CALL = "moveCall"
    PROGRAMMABLE_TRANSACTION = "programmableTransaction"


class Plugin(ABC):
    """Base plugin interface for transactions."""
    
    @abstractmethod
    def name(self) -> str:
        """Get the plugin name."""
        pass
    
    @abstractmethod
    def before_transaction(self, tx: dict, kind: str) -> None:
        """Called before transaction execution."""
        pass
    
    @abstractmethod
    def after_transaction(self, tx: dict, result: any, error: Exception) -> None:
        """Called after transaction execution."""
        pass
    
    @abstractmethod
    def build(self, tx: dict) -> None:
        """Called during transaction building."""
        pass


class NamedPackagesPlugin(Plugin):
    """Plugin for resolving named packages."""
    
    def __init__(self, packages: dict[str, str]):
        self.packages = packages
    
    def name(self) -> str:
        """Get the plugin name."""
        return "NamedPackagesPlugin"
    
    def before_transaction(self, tx: dict, kind: str) -> None:
        """Resolve named packages in the transaction."""
        pass
    
    def after_transaction(self, tx: dict, result: any, error: Exception) -> None:
        """No-op after transaction."""
        pass
    
    def build(self, tx: dict) -> None:
        """No-op during build."""
        pass
    
    def resolve(self, name: str) -> str:
        """Resolve a named package name to its address."""
        return self.packages.get(name, name)


class ValidatorPlugin(Plugin):
    """Plugin for validating transactions."""
    
    def __init__(self, validator: callable):
        self.validator = validator
    
    def name(self) -> str:
        """Get the plugin name."""
        return "ValidatorPlugin"
    
    def before_transaction(self, tx: dict, kind: str) -> None:
        """Validate the transaction."""
        if self.validator:
            self.validator(tx)
    
    def after_transaction(self, tx: dict, result: any, error: Exception) -> None:
        """No-op after transaction."""
        pass
    
    def build(self, tx: dict) -> None:
        """No-op during build."""
        pass


class PluginManager:
    """Manager for registering and executing plugins."""
    
    def __init__(self):
        self.plugins: list[Plugin] = []
    
    def register(self, plugin: Plugin) -> None:
        """Register a plugin."""
        self.plugins.append(plugin)
    
    def unregister(self, plugin: Plugin) -> None:
        """Unregister a plugin by instance."""
        if plugin in self.plugins:
            self.plugins.remove(plugin)
    
    def unregister_by_name(self, name: str) -> None:
        """Unregister a plugin by name."""
        self.plugins = [p for p in self.plugins if p.name() != name]
    
    def get(self, name: str) -> Plugin:
        """Get a plugin by name."""
        for plugin in self.plugins:
            if plugin.name() == name:
                return plugin
        return None
    
    def list(self) -> list[str]:
        """List all plugin names."""
        return [p.name() for p in self.plugins]
    
    def before_transaction(self, tx: dict, kind: str) -> None:
        """Call before_transaction on all plugins."""
        for plugin in self.plugins:
            plugin.before_transaction(tx, kind)
    
    def after_transaction(self, tx: dict, result: any, error: Exception) -> None:
        """Call after_transaction on all plugins."""
        for plugin in self.plugins:
            plugin.after_transaction(tx, result, error)
    
    def build(self, tx: dict) -> None:
        """Call build on all plugins."""
        for plugin in self.plugins:
            plugin.build(tx)