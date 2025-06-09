#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait NativeSend: Send {

}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> NativeSend for T where T: Send {
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait NativeSend {

}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> NativeSend for T {
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait NativeSync: Sync {

}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> NativeSync for T where T: Sync {
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait NativeSync {

}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> NativeSync for T {
}
