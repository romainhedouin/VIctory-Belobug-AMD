//! Platform-level data formats, shared by every Xbox Series title.
//!
//! Each module is pure (bytes in, bytes out, no I/O): [`texture`] (the
//! GPU's tiled texture layouts and copies to and from linear rows),
//! [`bcpack`] (BCPack texture streams, decoded) and [`xma`] (XMA2 audio,
//! decoded). Planned: the Opus audio codec and the zlib streams used by
//! hardware decompression.

pub mod bcpack;
pub mod texture;
pub mod xma;
