pub mod plugin;

pub use plugin::TransactionPlugin;

pub trait TransactionBuilder {
    fn add_plugin(&mut self, plugin: Box<dyn TransactionPlugin>);
    fn remove_plugin(&mut self, name: &str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_trait() {
        struct TestPlugin;

        impl TransactionPlugin for TestPlugin {
            fn name(&self) -> String {
                "test".to_string()
            }

            fn before_build(&mut self, _tx: &mut serde_json::Value) -> Result<(), String> {
                Ok(())
            }

            fn after_build(&mut self, _tx: &serde_json::Value) -> Result<(), String> {
                Ok(())
            }
        }

        let plugin: Box<dyn TransactionPlugin> = Box::new(TestPlugin {});
        assert_eq!(plugin.name(), "test");
    }
}