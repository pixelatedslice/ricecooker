fn main() {
    #[cfg(windows)]
    {
        embed_resource::compile("app.manifest", embed_resource::NONE)
            .expect("Failed to compile the Windows manifest. Ensure app.manifest exists.");
    }
}
