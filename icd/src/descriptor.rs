//! VkDescriptorSets device commands

use headers::vk_decls::*;
use log::warn;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::descriptor::*;
use runtime::logical_device::LogicalDevice;


pub unsafe extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSetLayout: Option<NonNull<VkDescriptorSetLayout>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let Some(bindings) = create_info.pBindings else {
        unreachable!()
    };
    let bindings = std::slice::from_raw_parts(bindings.as_ptr(), create_info.bindingCount as usize);

    let _ = pAllocator;

    let Some(pSetLayout) = pSetLayout else {
        unreachable!()
    };

    *pSetLayout.as_ptr() = DescriptorSetLayout::create(device, create_info.flags, bindings);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    DescriptorSetLayout::drop_handle(descriptorSetLayout);
}

pub unsafe extern "C" fn vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDescriptorPool: Option<NonNull<VkDescriptorPool>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let Some(pool_sizes) = create_info.pPoolSizes else {
        unreachable!()
    };
    let pool_sizes =
        std::slice::from_raw_parts(pool_sizes.as_ptr(), create_info.poolSizeCount as usize);

    let _ = pAllocator;

    let Some(pDescriptorPool) = pDescriptorPool else {
        unreachable!()
    };

    *pDescriptorPool.as_ptr() =
        DescriptorPool::create(device, create_info.flags, create_info.maxSets, pool_sizes);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    DescriptorPool::drop_handle(descriptorPool);
}

pub unsafe extern "C" fn vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkDescriptorSetAllocateInfo>>,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pAllocateInfo) = pAllocateInfo else {
        unreachable!()
    };
    let allocate_info = pAllocateInfo.as_ref();
    let Some(descriptorPool) = DescriptorPool::from_handle(allocate_info.descriptorPool) else {
        unreachable!()
    };
    let Some(pSetLayouts) = allocate_info.pSetLayouts else {
        unreachable!()
    };
    let set_layouts = std::slice::from_raw_parts(
        pSetLayouts.as_ptr(),
        allocate_info.descriptorSetCount as usize,
    );
    let Some(pDescriptorSets) = pDescriptorSets else {
        unreachable!()
    };
    let descriptor_sets = std::slice::from_raw_parts_mut(
        pDescriptorSets.as_ptr(),
        allocate_info.descriptorSetCount as usize,
    );

    for (set_layout, descriptor_set) in std::iter::zip(set_layouts, descriptor_sets) {
        *descriptor_set = DescriptorSet::create(device.clone(), descriptorPool.clone(), set_layout);
    }

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(_descriptorPool) = DescriptorPool::from_handle(descriptorPool) else {
        unreachable!()
    };

    let Some(pDescriptorSets) = pDescriptorSets else {
        unreachable!()
    };
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
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

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
