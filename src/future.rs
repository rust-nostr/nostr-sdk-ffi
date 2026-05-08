#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn assume_send<F>(future: F) -> WasmSend<F>
where
    F: Future,
{
    WasmSend(future)
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn assume_send<F>(future: F) -> F
where
    F: Future + Send,
{
    future
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) struct WasmSend<F>(F);

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
// SAFETY: wasm32-unknown-unknown is used here as a single-threaded UniFFI target.
// The wrapper is only awaited locally and never moves the inner future between threads.
unsafe impl<F> Send for WasmSend<F> {}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<F> Future for WasmSend<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // SAFETY: pinning is projected from the wrapper to its only field.
        // The field is not moved while pinned.
        unsafe { self.map_unchecked_mut(|wrapper| &mut wrapper.0) }.poll(cx)
    }
}
