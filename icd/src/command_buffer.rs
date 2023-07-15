//! VkCommandBuffer device commands

use headers::vk_decls::*;
use runtime::buffer::Buffer;
use runtime::command_buffer::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::image::Image;
use runtime::logical_device::LogicalDevice;
use runtime::pipeline::{Framebuffer, Pipeline, PipelineLayout, RenderPass};
use runtime::*;

pub unsafe extern "C" fn vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCommandPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCommandPool: Option<NonNull<VkCommandPool>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pCommandPool) = pCommandPool else {
        unreachable!()
    };

    *pCommandPool.as_ptr() = CommandPool::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    CommandPool::drop_handle(commandPool);
}

pub unsafe extern "C" fn vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkCommandBufferAllocateInfo>>,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) -> VkResult {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pAllocateInfo) = pAllocateInfo else {
        unreachable!()
    };
    let allocate_info = pAllocateInfo.as_ref();

    let Some(pCommandBuffers) = pCommandBuffers else {
        unreachable!()
    };

    let command_buffer_count = allocate_info.commandBufferCount as usize;
    let command_buffers = vec![CommandBuffer::create(allocate_info); command_buffer_count]
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>();
    std::ptr::copy_nonoverlapping(
        command_buffers.as_ptr(),
        pCommandBuffers.as_ptr(),
        command_buffer_count,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(_commandPool) = CommandPool::from_handle(commandPool) else {
        unreachable!()
    };

    pCommandBuffers
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), commandBufferCount as usize)
        })
        .iter()
        .for_each(|&handle| CommandBuffer::drop_handle(handle));
}

pub unsafe extern "C" fn vkBeginCommandBuffer(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: Option<NonNull<VkCommandBufferBeginInfo>>,
) -> VkResult {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(pBeginInfo) = pBeginInfo else {
        unreachable!()
    };
    let _ = pBeginInfo.as_ref();

    commandBuffer.lock().begin();

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    commandBuffer.lock().end();

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkCmdPipelineBarrier(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: Option<NonNull<VkMemoryBarrier>>,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: Option<NonNull<VkBufferMemoryBarrier>>,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: Option<NonNull<VkImageMemoryBarrier>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let _ = srcStageMask;
    let _ = dstStageMask;
    let _ = dependencyFlags;
    let _ = memoryBarrierCount;
    let _ = pMemoryBarriers;
    let _ = bufferMemoryBarrierCount;
    let _ = pBufferMemoryBarriers;
    let _ = imageMemoryBarrierCount;
    let _ = pImageMemoryBarriers;

    commandBuffer.lock().cmd_pipeline_barrier();
}

pub unsafe extern "C" fn vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: Option<NonNull<VkRenderPassBeginInfo>>,
    contents: VkSubpassContents,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(pRenderPassBegin) = pRenderPassBegin else {
        unreachable!()
    };
    let render_pass_begin = pRenderPassBegin.as_ref();
    let Some(render_pass) = RenderPass::from_handle(render_pass_begin.renderPass) else {
        unreachable!()
    };
    let Some(framebuffer) = Framebuffer::from_handle(render_pass_begin.framebuffer) else {
        unreachable!()
    };
    let clear_values = render_pass_begin.pClearValues.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), render_pass_begin.clearValueCount as usize)
    });

    commandBuffer.lock().cmd_begin_render_pass(
        render_pass,
        framebuffer,
        render_pass_begin.renderArea,
        clear_values,
        contents,
    );
}

pub unsafe extern "C" fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    commandBuffer.lock().cmd_end_render_pass();
}

pub unsafe extern "C" fn vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(pipeline) = Pipeline::from_handle(pipeline) else {
        unreachable!()
    };

    commandBuffer
        .lock()
        .cmd_bind_pipeline(pipelineBindPoint, pipeline);
}

pub unsafe extern "C" fn vkCmdBindDescriptorSets(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
    dynamicOffsetCount: u32,
    pDynamicOffsets: Option<NonNull<u32>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(pipeline_layout) = PipelineLayout::from_handle(layout) else {
        unreachable!()
    };

    let descriptor_sets = pDescriptorSets.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), descriptorSetCount as usize)
    });

    let dynamic_offsets = pDynamicOffsets.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), dynamicOffsetCount as usize)
    });

    commandBuffer.lock().cmd_bind_descriptor_sets(
        pipelineBindPoint,
        pipeline_layout,
        firstSet,
        descriptor_sets,
        dynamic_offsets,
    );
}

pub unsafe extern "C" fn vkCmdBindVertexBuffers(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: Option<NonNull<VkBuffer>>,
    pOffsets: Option<NonNull<VkDeviceSize>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let buffers = pBuffers.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), bindingCount as usize)
    });

    let offsets = pOffsets.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), bindingCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_bind_vertex_buffers(firstBinding, buffers, offsets);
}

pub unsafe extern "C" fn vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: Option<NonNull<VkViewport>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let viewports = pViewports.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), viewportCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_set_viewport(firstViewport, viewports);
}

pub unsafe extern "C" fn vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: Option<NonNull<VkRect2D>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let scissors = pScissors.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), scissorCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_set_scissors(firstScissor, scissors);
}

pub unsafe extern "C" fn vkCmdDraw(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    commandBuffer
        .lock()
        .cmd_draw(vertexCount, instanceCount, firstVertex, firstInstance);
}

pub unsafe extern "C" fn vkCmdCopyBufferToImage(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferImageCopy>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(srcBuffer) = Buffer::from_handle(srcBuffer) else {
        unreachable!()
    };

    let Some(dstImage) = Image::from_handle(dstImage) else {
        unreachable!()
    };

    let regions = pRegions.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), regionCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_copy_buffer_to_image(srcBuffer, dstImage, dstImageLayout, regions);
}

pub unsafe extern "C" fn vkCmdCopyImageToBuffer(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferImageCopy>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(srcImage) = Image::from_handle(srcImage) else {
        unreachable!()
    };

    let Some(dstBuffer) = Buffer::from_handle(dstBuffer) else {
        unreachable!()
    };

    let regions = pRegions.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), regionCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_copy_image_to_buffer(srcImage, dstBuffer, srcImageLayout, regions);
}

pub unsafe extern "C" fn vkCmdCopyBuffer(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferCopy>>,
) {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(srcBuffer) = Buffer::from_handle(srcBuffer) else {
        unreachable!()
    };

    let Some(dstBuffer) = Buffer::from_handle(dstBuffer) else {
        unreachable!()
    };

    let regions = pRegions.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), regionCount as usize)
    });

    commandBuffer
        .lock()
        .cmd_copy_buffer_to_buffer(srcBuffer, dstBuffer, regions);
}
