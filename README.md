# VkSWR

Vulkan ICD written in Rust.

The current goal is to fully implement the Vulkan Core 1.0 API with a software renderer.

## Goals
- [x] vulkaninfo
- [ ] vkcube
  - [x] Entry points.
  - [ ] Rendering.
  - [x] Presentation.
- [ ] dEQP-VK tests
  - [ ] dEQP-VK.api.*
    - [x] dEQP-VK.api.info.*
    - [x] dEQP-VK.api.smoke.triangle
      - [x] Triangle list rasterization.
      - [x] Vertex shader.
      - [x] Fragment shader.

        ![Result](assets/deqp-vk/dEQP-VK.api.smoke.triangle_Result.PNG)![Reference](assets/deqp-vk/dEQP-VK.api.smoke.triangle_Reference.PNG)![ErrorMask](assets/deqp-vk/dEQP-VK.api.smoke.triangle_ErrorMask.PNG)
    - [x] dEQP-VK.api.object_management.single.buffer_view_uniform_r8g8b8a8_unorm
  - [ ] dEQP-VK.memory.*
    - [x] dEQP-VK.memory.requirements.*
    - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_2
    - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_4
    - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.8192_vertex_buffer_stride_2
    - [x] dEQP-VK.memory.pipeline_barrier.graphics.host_write_index_buffer.1024
    - [ ] dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024

      ![Result](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.host_write_uniform_buffer.1024_Result.PNG)![Reference](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.host_write_uniform_buffer.1024_Reference.PNG)![ErrorMask](assets/deqp-vk/dEQP-VK.memory.pipeline_barrier.host_write_uniform_buffer.1024_ErrorMask.PNG)

## Testing

The tests do not use Vulkan tools from the user's `PATH`, and they do not read
`VULKAN_SDK` or `VULKAN_CTS_PATH`. Tools are cached in `target/vkswr-tools`:

- Tests that need `glslangValidator`, `vulkaninfo`, or `vkcube` download and
  extract the pinned Vulkan SDK on first use.
- Tests that need `deqp-vk` download the pinned Vulkan CTS sources and build
  `deqp-vk` on first use.

- Vulkan SDK `1.4.363.0` (LunarG) provides `glslangValidator`, `vulkaninfo`, and `vkcube`.
- Vulkan CTS `vulkan-cts-1.4.7.0` (Khronos) is built from source.

The first test run therefore needs network access plus `git`, `python3` (with
`lxml`), `cmake`, `ninja`, and a C++ toolchain. Only Linux is supported by the
download helper.

Environment overrides:

- `VKSWR_TOOLS_DIR` — where downloads and builds are cached (default:
  `target/vkswr-tools`).
- `VKSWR_CTS_JOBS` — parallelism for the CTS build.
- `VKSWR_PYTHON` / `VKSWR_CMAKE` — override the `python3`/`cmake` executables.

`cargo clean` removes the cache, so the next test run downloads and builds the
tools again.

Tests that exercise features which are not implemented yet are marked
`#[ignore]` with an explanation so the default `cargo test` run stays green:

- `run_vkcube` — needs matrix shader support and texture sampling.
- `run_deqp_vk_memory_pipeline_barrier_host_write_uniform_buffer_1024` — needs
  descriptor-set/uniform-buffer support (`vkUpdateDescriptorSets` and
  `vkCmdBindDescriptorSets` are still no-ops).
