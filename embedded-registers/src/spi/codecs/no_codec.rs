use crate::{ReadableRegister, WritableRegister};

/// A codec that represents absense of a codec. This has two main usecases:
///
/// Firstly, if this is used as the default codec for a device, it essentially
/// requires any associated register to explicitly specify a codec. Otherwise
/// accessing that register via the [`RegisterInterfaceSync`](crate::RegisterInterfaceSync)
/// or [`RegisterInterfaceAsync`](crate::RegisterInterfaceAsync) trait will cause a panic.
///
/// Secondly, specifying this codec as the default for a register will cause
/// any reads or writes to that register via the [`RegisterInterfaceSync`](crate::RegisterInterfaceSync)
/// or [`RegisterInterfaceAsync`](crate::RegisterInterfaceAsync) traits to be performed
/// to be performed through the default codec of the device.
pub struct NoCodec {}

#[maybe_async_cfg::maybe(
    idents(hal(sync = "embedded_hal", async = "embedded_hal_async"), Codec),
    sync(feature = "sync"),
    async(feature = "async"),
    keep_self
)]
impl crate::spi::Codec for NoCodec {
    #[inline]
    async fn read_register<R, I>(_interface: &mut I) -> Result<R, I::Error>
    where
        R: ReadableRegister,
        I: hal::spi::r#SpiDevice,
    {
        panic!("spi::codecs::NoCodec cannot be used at runtime! Please specify a real codec to access this register.");
    }

    #[inline]
    async fn write_register<R, I>(_interface: &mut I, _register: impl AsRef<R>) -> Result<(), I::Error>
    where
        R: WritableRegister,
        I: hal::spi::r#SpiDevice,
    {
        panic!("spi::codecs::NoCodec cannot be used at runtime! Please specify a real codec to access this register.");
    }
}
