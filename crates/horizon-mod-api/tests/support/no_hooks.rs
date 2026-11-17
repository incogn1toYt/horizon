// Implementacja zastępcza dla natywnych testów.
#[unsafe(no_mangle)]
fn __horizon_mod_hooks() -> Box<dyn mod_api::Hooks> {
    struct Empty;

    impl mod_api::Hooks for Empty {
        fn subscribed(&self) -> &'static [Option<mod_api::HookId>] {
            &[]
        }
    }

    Box::new(Empty)
}
