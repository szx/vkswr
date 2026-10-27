# VkSWR

Vulkan ICD written in Rust.

The current goal is to fully implement the Vulkan Core 1.0 API with a software renderer.

## Goals

- [x] vulkaninfo
- [ ] vkcube
  - [x] Entry points.
  - [ ] Rendering.
  - [x] Presentation.

### dEQP-VK

<!-- goals:begin -->
- [?] dEQP-VK.api.*
  - [x] dEQP-VK.api.device_init.create_device.basic
  - [x] dEQP-VK.api.device_init.create_device_unsupported_features.core
  - [?] dEQP-VK.api.device_init.create_instance_device_intentional_alloc_fail.basic
  - [x] dEQP-VK.api.info.*
  - [x] dEQP-VK.api.object_management.single.buffer_view_uniform_r8g8b8a8_unorm
  - [x] dEQP-VK.api.smoke.create_shader
  - [x] dEQP-VK.api.smoke.triangle
    ![Result](assets/deqp-vk/dEQP-VK.api.smoke.triangle_Result.PNG)![Reference](assets/deqp-vk/dEQP-VK.api.smoke.triangle_Reference.PNG)![ErrorMask](assets/deqp-vk/dEQP-VK.api.smoke.triangle_ErrorMask.PNG)
  - [x] dEQP-VK.api.version_check.entry_points
- [?] dEQP-VK.memory.*
  - [x] dEQP-VK.memory.mapping.suballocation.random.0
  - [x] dEQP-VK.memory.mapping.suballocation.random.3
  - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_index_buffer.1024
  - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_transfer_src.1024
  - [?] dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024
    ![Result](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024_Result.PNG)![Reference](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024_Reference.PNG)![ErrorMask](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024_ErrorMask.PNG)
  - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_2
  - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_4
  - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.8192_vertex_buffer_stride_2
  - [x] dEQP-VK.memory.requirements.*
    - [x] dEQP-VK.memory.requirements.core.buffer.regular
    - [x] dEQP-VK.memory.requirements.core.image.regular_tiling_linear
<!-- goals:end -->

## Implementation progress

The dEQP-VK checklist above is generated, not hand-edited:

- `test_suite/tests/integration_tests.rs` is the inventory: every `#[test]`
  that calls `run_deqp_vk` is one entry, and a case ending in `*` runs a whole
  group.
- `deqp-results.tsv` records the outcome of each case from the last run. The
  test suite rewrites it, so commit it together with `README.md`.
- `cargo run -p xtask -- readme` regenerates the region between the `goals`
  markers; add `--check` to fail instead of writing.

## Sources of truth

Sources:
- `codegen/vk.xml`
- `codegen/src/policy.rs`
- `rspirv`
- `test_tools/src/lib.rs`

Generated sources live in `OUT_DIR`.

## Testing

Vulkan SDK is downloaded and provides `glslangValidator`, `vulkaninfo`, and `vkcube`.
Vulkan CTS is built from source and provides `deqp-vk`.

Environment overrides:
- `VKSWR_TOOLS_DIR` - where downloads and builds are cached (default:
  `target/vkswr-tools`).
- `VKSWR_CTS_JOBS` - parallelism for the CTS build.
- `VKSWR_PYTHON` / `VKSWR_CMAKE` - override the `python3`/`cmake` executables.
