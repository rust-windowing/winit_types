#![cfg(feature = "serde_feature")]

use serde::{Deserialize, Serialize};
use winit_types::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};

#[allow(dead_code)]
fn needs_serde<S: Serialize + Deserialize<'static>>() {}

#[test]
fn dpi_serde() {
    needs_serde::<LogicalPosition<f64>>();
    needs_serde::<PhysicalPosition<i32>>();
    needs_serde::<PhysicalPosition<f64>>();
    needs_serde::<LogicalSize<f64>>();
    needs_serde::<PhysicalSize<u32>>();
}
