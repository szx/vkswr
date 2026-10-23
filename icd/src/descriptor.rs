//! VkDescriptorSets device commands

use headers::vk_decls::*;
use log::warn;
use runtime::context::VulkanObject;
use runtime::descriptor::*;
use runtime::logical_device::LogicalDevice;

pub unsafe extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSetLayout: Option<NonNull<VkDescriptorSetLayout>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    let bindings = create_info.pBindings.expect("value is present");
    let bindings = std::slice::from_raw_parts(bindings.as_ptr(), create_info.bindingCount as usize);

    let _ = pAllocator;

    let pSetLayout = pSetLayout.expect("Vulkan guarantees a non-null pointer");

    *pSetLayout.as_ptr() = DescriptorSetLayout::create(device, create_info.flags, bindings);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    DescriptorSetLayout::drop_handle(descriptorSetLayout);
}

pub unsafe extern "C" fn vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDescriptorPool: Option<NonNull<VkDescriptorPool>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    let pool_sizes = create_info
        .pPoolSizes
        .expect("Vulkan guarantees a non-null pointer");
    let pool_sizes =
        std::slice::from_raw_parts(pool_sizes.as_ptr(), create_info.poolSizeCount as usize);

    let _ = pAllocator;

    let pDescriptorPool = pDescriptorPool.expect("Vulkan guarantees a non-null pointer");

    *pDescriptorPool.as_ptr() =
        DescriptorPool::create(device, create_info.flags, create_info.maxSets, pool_sizes);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    DescriptorPool::drop_handle(descriptorPool);
}

pub unsafe extern "C" fn vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkDescriptorSetAllocateInfo>>,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pAllocateInfo = pAllocateInfo.expect("Vulkan guarantees a non-null pointer");
    let allocate_info = pAllocateInfo.as_ref();
    let descriptorPool = DescriptorPool::from_handle(allocate_info.descriptorPool)
        .expect("Vulkan handle refers to a live object");
    let pSetLayouts = allocate_info
        .pSetLayouts
        .expect("Vulkan guarantees a non-null pointer");
    let set_layouts = std::slice::from_raw_parts(
        pSetLayouts.as_ptr(),
        allocate_info.descriptorSetCount as usize,
    );
    let pDescriptorSets = pDescriptorSets.expect("Vulkan guarantees a non-null pointer");
    let descriptor_sets = std::slice::from_raw_parts_mut(
        pDescriptorSets.as_ptr(),
        allocate_info.descriptorSetCount as usize,
    );

    for (set_layout, descriptor_set) in std::iter::zip(set_layouts, descriptor_sets) {
        *descriptor_set =
            DescriptorSet::create(device.clone(), descriptorPool.clone(), *set_layout);
    }

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _descriptorPool =
        DescriptorPool::from_handle(descriptorPool).expect("Vulkan handle refers to a live object");

    let pDescriptorSets = pDescriptorSets.expect("Vulkan guarantees a non-null pointer");
    let descriptor_sets =
        std::slice::from_raw_parts(pDescriptorSets.as_ptr(), descriptorSetCount as usize);

    for descriptor_set in descriptor_sets {
        DescriptorSet::drop_handle(*descriptor_set);
        warn!("TODO: Remove from DescriptorPool in DescriptorSet::drop()");
    }

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkUpdateDescriptorSets(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: Option<NonNull<VkWriteDescriptorSet>>,
    descriptorCopyCount: u32,
    pDescriptorCopies: Option<NonNull<VkCopyDescriptorSet>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let descriptor_writes = pDescriptorWrites
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), descriptorWriteCount as usize));
    let descriptor_copies = pDescriptorCopies
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), descriptorCopyCount as usize));

    if let Some(_descriptor_write) = descriptor_writes {
        warn!("TODO: Descriptor write")
    }

    if let Some(_descriptor_copy) = descriptor_copies {
        warn!("TODO: Descriptor copy")
    }
}
