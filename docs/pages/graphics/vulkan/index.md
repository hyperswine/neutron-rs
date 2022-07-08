---
layout: default
title: Vulkan
has_children: true
parent: Graphics
---

## Vulkan

Vulkan is prob one of the greatest APIs ever. Gonna talk about [this](https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Instance).

### Resource Management

Every vulkan object we create must be explicitly destroyed. Like malloc/free. I think this is referring to RAM objects, not VRAM objects. Although I guess we can use smart pointers for both.

### Window

I think there's surface functions for makign windows. Maybe use the OS' underlying window system, or without a client window. So it wont be immediately draggable/resizable maybe. I also dunno how it would render over the DE. Maybe when it is foregrounded it just draws over everything else.

I dunno exactly. Maybe we can build the DE in vulkan itself with windowing integrated within it. If you want to use the window system directly, you have to interface with the OS with a syscall or window library that implements vulkan/glfw window creation it seems.

I think VkViewport auto handles all the surface changes for us. Its also true for a lot of the state we specify. We dont have to recreate the pipeline, just change it on the fly.

### GLFW

GLFW actually works pretty well for vulkan. You can use `glfwGetRequiredInstanceExtensions(&count)` to get a list of supported vulkan extensions.

### Objects

Vulkan objects generally have a `sType` which makes you respecify the object's type for some reason. Also `pNext` to point to extension info in the future. Most objects are makable through functions. They will need a `createInfo` structure though, which you need to make manually from a struct.

Functions to create objects usually have 3 types of args:

1. pointer to struct with how to create the object
2. pointer to custom allocator callbacks. Prob nullptr in most cases
3. pointer to variable (array/int) storing the handle to the new object

It then returns a `VkResult` which is either `VK_SUCCESS` or some other enumerated error code. Good to just `assert(vkDoSomething() == VK_SUCCESS)`.

Object handles are usually structs. They are defined by the macro `VK_DEFINE_HANDLE(object) typedef struct object##_T* object;`. This basically just makes an object handle like `VkPhysicalDevice` a `struct VkPhysicalDevice_T* VkPhysicalDevice`. The real info is already defined in `VkPhysicalDevice_T` in the vulkan implementation. Our handle just points to a structure that the OS driver/userspace driver takes care of.

### Extensions

To query the extension details, we use `vkEnumerateInstanceExtensionProperties(null, &count, &vec)`. Then you iterate over `vec` to ensure the required extensions are available. A quick way to ensure glfw compatibility is to compare the output with `glfwGetRequiredInstanceExtensions(&count) -> const char**`.

NOTE: if not all extensions needed by glfw are available, you can still use vulkan for compute/non rendering workloads. As long as vulkan is actually supported on the platform.

## Shaders

Vulkan shaders are prob quite similar to GL shaders. And compile to SPIR-V mostly. SPIR-V is the bytecode format that vendors develop for. So they dont have to misinterpret GLSL. Instead we write GLSL using some khronos format and compile it to SPIR-V, which then can be compiled further into the GPU's native ISA or interpreted. I think AOT is better.

`glslc` is usually the compiler used. And supports nice features like include directives and a familiar gcc-like cli. And prob other directives that gcc/clang supports maybe. Like opengl, we have a `main` function that runs on our inputs for each stage. If there is one. And output stuff, if there is one.

NOTE: vulkan uses the same z-coord scheme as Direct3D from 0 to 1. Instead of any number. Its also different from OpenGL in NDC as the y values are flipped. Top left is actually (-1, -1). Idk why, but ok. Maybe its better when you apply transformations to the matrices.

After we're done converting vertices in model space to screen space (MVP matrix), we end up with NDC which can be piped into the fragment shader. Before that it implicitly rasterises it through converting each vertex into a fragment (pixel) for the framebuffer. Then for each pixel we can sample a texture or just color each pixel in according to some scheme. I think it interpolates linearly by default. Like OpenGL, you can choose some interpolation scheme too.

We can either compile the shaders manually via cli or through `libshaderc` to do it at runtime.

To create a shader module, create a `VkShaderModuleCreateInfo` struct and a `VkShaderModule` handle. Then call `vkCreateShaderModule(device, create_info, allocator, *VkShaderModule) -> VKResult`. We would call for each of our shaders. Given that we have already compiled them. We then create shader stages, Vertex stage and a fragment stage. With `VkPipelineShaderStageCreateInfo` struct and `

## Drawing a triangle

Like opengl, we have a main render loop. Where we can use glfw to poll for events and act on them. Change game logic in RAM. Then use the new logic to create a scene or buffer object that represents the new scene. Then shade it in.

The idea is to create an instance, query extensions, setup validation layers, setup queues. Then setup a window, swapchain and view.

### Instances

The first thing though, is to create an `instance` with `createInstance() -> VKInstance`. Which is a connection between the program and the vulkan library. Prob like a conn between pid and the vulkan graphics driver / file in `/dev/gpu`. Then we specify the vulkan global extensions and validation we want to use. Global means it doesnt matter whether a specific GPU supports it or not, so i guess we have to check later. To do so we create a `VkInstanceCreateInfo` struct.

We can also create a VKApplicationInfo and specify extra program metadata. This is optional but may allow some optimisations by allowing vulkan to choose a suitable engine.

After an instance is created, you have to actually create a vk instance with `vkCreateInstance(&createInfo, nullptr, &instance)`.

We should only destroy an instance right before the program exits. Use `vkDestroyInstance(instance, callback)`.

### Validation Layers

Optional components that hook into vulkan function calls to apply additional operations like checking the values of params against the specification to detect misuse. Tracking creation/destruction of objects to find resource leaks. We can also check for thread safety by tracking the threads the alls originate from. And logging every call/params to stdout. Similarly trracing vulkan calls for profiling and replaying.

Its good to enable them for debug builds but disable them for release builds. You have to use LunarG SDK as vulkan doesnt provide any layers builtin. All the standard validation is bundled into a layer in the SDK known as `VK_LAYER_KHRONOS_validation`.

The only type of validation is instance validation, that applies to a vulkan connection. No device specific validation.

The first thing is to use `vkEnumerateInstanceLayerProperties(&layerCount, vec)`. The `vec` is of type `Vec<VkLayerProperties>`. Once done we then check whether `vec` contains our wanted layers. We can compare it by ticking off each layer in our `wantedLayers`, which may include `"VK_LAYER_KHRONOS_validation"` for example. If `vec` contains that, then we can tick it off. If we dont find a specific layer we want, then we can throw an exception.

Then we can plug the whole function into our create instance function, which checks if the validation layers exist. If it does then we can proceed to call vkCreateInstance. Note we should also specify createInfo.enabledLayerCount and createInfo.ppEnabledNames to our returned vec data. vkCreateInstance should also not return `VK_ERROR_LAYER_NOT_PRESENT`.

### Message Callbacks

We can actually specify a callback handler for a validation layer. So we dont have to actually use the provided implementation. We can also decide which messages to see, e.g. not all errors are fatal. Use the `"VK_EXT_debug_utils"` extension to setup a debug messenger with a callback. We should actually specify it as `VK_EXT_DEBUG_UTILS_EXTENSION_NAME` macro to avoid typos.

With debug callbacks, we are provided a nice `pCallbackData` param to print out the debug message of the log, the array of vulkan object handles related to the log, and the number of objects in the array. We also have a`pUserData` to allow us to pass our own data to it.

Note to call the function `vkCreateDebugUtilsMessengerEXT` that actually sets up a custom callback with the validation layer, we need to call it through its extension function. With `vkGetInstanceProcAddr` to look up that function's addr. We also have to cast it to a function pointer `(PFN_vkCreateDebugUtilsMessengerEXT) vkGetInstanceProcAddr` like all extension functions. That then returns the extension function we want, which we can set to `auto` and call directly.

Note the vkCreateDebugUtilsMessengerEXT takes in an `instance` as its first param. This is true for all child objects, either part of an instance or something else. Also we cant debug if something went wrong in `vkCreateInstance` and `vkDestroyInstance` calls. Because the debug layer requires those calls to be made before/after. We can solve this with `pNext` by passing a pointer to a separate `VkDebugUtilsMessengerCreateInfoEXT` for those functions. It can get quite big because we also have to create a `createInfo` for it.

[Here](https://github.com/KhronosGroup/Vulkan-ValidationLayers/blob/master/layers/vk_layer_settings.txt) tells us how to configure the behavior of the validation layer.

### Select Physical Device

An actual graphics card that supports vulkan. We use a `VkPhysicalDevice` handle. To list the available graphics cards, use `vkEnumeratePhysicalDevices(instance, &deviceCount, vec)`. This will then give us a filled array of VkPhysicalDevice.

We can pick a suitable device from that list by checking for properties. Do it with `VkPhysicalDeviceProperties{}` and `vkGetPhysicalDeviceProperties(device, &deviceProperties)`. This will return a list of properties which we can check for, e.g. texture compression, 64-bit floats and multi viewport rendering. We could check like `deviceProperties.deviceType == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU && deviceFeatures.geometryShader == true`.

### Queue

Almost all operations in vulkan involve submitting a command to a queue. There are different types of queues from different queue families. Each queue family only allows a subset of commands. E.g. the queue for processing compute commands only processes those types of commands. Other queues include memory transfer queues and render queues.

To retrieve a list of queue families, we use `vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, vec)`. Note that `device` is the parent object. Sometimes a queue may not be available, like a specific dedicated transfer family queue. The vec contains `VkQueueFamilyProperties`. You can test each of their `.queueFlags &` with a specific queue bit in order to tick off each queue family you want. Usually we want at least a queue family that supports `VK_QUEUE_GRAPHICS_BIT`.

To actually create the queues, we first specify which queue family we want, and how many queues we want for it. With `VkDeviceQueueCreateInfo{sType, pNext, flags, queueFamilyIndex, queueCount,pQueuePriorities}` as the method of creation. Most of the time one queue is enough. Most drivers only support a few queues. What you do is spawn multiple user threads that all create command buffers (producer-consumer mutex) by pushing it into a single buffer. Then submit them all at once to the queue. Or maybe submit the queue all at once?

Also you should assign a priority to a queue (queue family im pretty sure) between 0-1. This allows us to influence the scheduling of command buffer execution for multiple queue families wanting to execute a command.

### Logical Device

We have to setup a logical device to interface with a chosen physical device. To do so we create a `VkDevice` handle for it. Also we should specify which queues to create after we've queried the queues available. First we create our struct `VkDeviceCreateInfo{}` and add pointers to the queue creation info and device features. So we reuse the queue creation info here so we can tell a specific gpu what to do.

Then we just call `vkCreateDevice(physicalDevice, &createInfo, nullptr, &device)`. And like other important `vk` functions that return a value, we should assert `VK_SUCCESS`. Before exiting, we should also destroy our logical device with `vkDestroyDevice(device, null)`.

To retrieve a queue handle from a queue, we first create a handle `VkQueue`. Then call `vkGetDeviceQueue(device, queueFamilyIndex, queueIndex, &VkQueue)`.

### Window Surface

We've established a conn with our program and the vulkan library. But we do not have a connection with the OS' windowing system. Since vulkan is platform agnostic, it needs to be able to call the OS' functions that it has impl'd for.

A surface is an abstract context to render a frame into. GLFW's required extensions already selects `VK_KHR_surface` which is needed to interface with the OS window system.

NOTE: we dont actually need a surface if we want. We can still use the GPU to render but just ditch the result or save it as a gif or something.

The first thing is to create a `VkSurfaceKHR` handle. GLFW handles the platform differences like `VK_KHR_win32_surface` for windows for us. If we didnt use GLFW, then we would have to use something like `VK_KHR_win32_surface`. The platform native functions can also be included with GLFW with `#include <GLFW/glfw3native.h>`. The window surface is a vulkan object so it comes with a create info struct `VkWin32SurfaceCreateInfoKHR`. This has two key fields, `hwnd` and `hinstance`. With windows your hinstance needs to be fetched with `GetModuleInstace`.

Or we can just call `glfwCreateWindowSurface(instance, window, nullptr, &surface) -> VK_STATUS`. We can then destroy the surface with `vkDestroySurfaceKHR(instance, surface, nullptr)`. Note a surface is always bound to an instance. So if you have multiple windows (surfaces) open, you have to destroy each individually.

Now just because the vulkan implementation supports the window system, not every physical device can present images to the surface. Maybe theres a GPU that does not have an output display connected to it. So we should check if there is a QueueFamily that supports `vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface, &presentSupport)`. If `presentSupport` is true, then the gpu can render to the surface we want. So if we created a surface on monitor 0, that gpu would prob be connected to it. In practice, the surface queuefamily would prob be the same as the graphics command queuefamily. We could prob create two queues but one queue is enough. We still treat them as two separate queues though.

We first create the presentation queue handle `VkQueue`. And retrieve the queue handle for it `vkGetDeviceQueue(device, queueFamilyIndex, queueIndex, &presentQueue)`. Note queueIndex is usually 0. If the queue families are the same, then our two queue handles will have the same value. But it doesnt matter.

### Swapchain

The main idea is to be able to synchronise the presentation of images to the refresh rate of the screen. So somehow maintain 60fps, either by skipping a render cycle or not rendering just yet until the next tick. To help the render catch up or if its too fast, keep up.

Swapchains / image presentation is heavily tied to the window system. And the surfaces associated with each window. To be able to use swapchains, enable the `VK_KHR_swapchain` after querying its support. The extension macro is `VK_KHR_SWAPCHAIN_EXTENSION_NAME`, which can be queried along with other extensions in a vec. And also change anything else that would be affected like the logical device create struct.

Not all devices support all of the swapchain properties. We usually want basic surface capabilities, color and pixel format, and presentation modes. We create a `VkSurfaceCapabilitiesKHR` handle and use it for `vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, *VkSurfaceCapabilitiesKHR) -> VkResult`. This just queries for the basic capabilities. We also want to query color formats of the surface through `vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, *surfaceFormatCount, *VkSurfaceFormatKHR)`. This can be done with the same args mostly. And presentation modes with `vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice, surface, *presentationModeCount, *VkSurfaceFormatKHR) -> VkResult`. This can all be put into the function to query whether the physical device is suitable for our needs.

Lets say we want to choose a suitable color format. We should select a `VkSurfaceFormatKHR` from the vec we passed to the function. E.g. we may want `VK_FORMAT_B8G8R8A8_SRGB` format and a `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR` color space. We and each of their bits until we find the right VkSurfaceFormatKHR with those bits. Then we can kick the rest out.

Other settings include the conditions for swapping images to the screen and the resolution of images in the swap chain. For the conditions, we usually want `VK_PRESENT_MODE_FIFO_KHR`, which is guarenteed to be available for a device. And simply works like a normal FIFO of images sync'd with the refresh rate. If queue is full the program has to wait or block until it can push more images. This is basically vertical sync where the moment the display is refreshed to the image is vblank. Or is it good? Also `VK_PRESENT_MODE_MAILBOX_KHR` is pretty good. Its basically triple buffering and replaces images in the queue with new ones if its full. Reduces latency and does not have tearing. We can also have `VK_PRESENT_MODE_IMMEDIATE_KHR` which transfers the rendered image to the screen right after its done. Can result in half finished frames / tearing.

The last one is the swap extent. The res is usually equal to the surface size in width and height. We create a `VkExtent2D` handle and call `glfwGetFramebufferSize(window, &width, &height)` to get the width and height of the window. Then we can set the handle to the width and height, given that it is at most the size supported by the swapchain. If its more, then it wont work. So we can check the `VkSurfaceCapabilitiesKHR` for `maxImageExtent` and `minImageExtent` sizes and clamp between them. Then if the surface/window is resized with a glfw hotkey (or simply resized directly?) through poll_events, we can call a callback that resets VkExtent2D and calls its associated function?

To create the swapchain, we create a `VkSwapchainCreateInfoKHR` struct. Another thing is the queue image sharing mode. Should images be owned by a specific queue or be shared concurrently? If queue families differ, then we should use concurrent. If its the same queue family, then just let one of them own it. We can also allow transformation of the image, like, flipped by 90deg, etc. Before outputting. Select no transformations with `VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR`, which also ignores the alpha channel. Im pretty sure monitors dont have an alpha channel and its mostly only used as a depth buffer.

We create a swapchain handle with `VkSwapchainKHR` then call `vkCreateSwapchainKHR(logical_device, createInfo, allocator_callback, *VkSwapchainKHR) -> VkResult`. You also need to destroy the swapchain `vkDestroySwapchainKHR(device, swapChain, nullptr)` later. NOTE: in a simple triangle app we would submit our commands to be rendered with the graphics queue family. And then submit the rendered frame to be outputted with a presentation queue family. Though on most hardware the graphics queue family and pres queue family are the same, and with only a single queue. What does this mean? We prob dont need to use concurrent mode and just have it mostly simply done for us.

How do we get the images from the swapchain so we can then manipulate them (post processing) and output them? I think there may be some vk function to tell it to output to the surface right away. But we can also do it manually through `vkGetSwapchainImagesKHR(device, swapChain, &imageCount, vec)`, which will give us the images. We could also resize the array depending on how many images there are. But idk is that a good idea? why not just allocate a vec on the stack then with a max size. Because there is no max size. So one way is to just call it without a vec first to get the count, resize then call it again with the resized vec. This gets us the images in RAM though. So we can then go through another pipeline or maybe theres some better idea.

Also store the format and extent handles so we can use them again. As mentioned above with `VkExtent2D`.

### Image View

To use any `VkImage` like those in the swapchain, we create a `VkImageView` object. We would be able to use the view in the pipeline. To create it, we create a `VkImageViewCreateInfo` struct and fill it with macros and images like `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO` and `swapChainImages[i]` for each image in the swapchain. We can also specify the `format` field which treats the image as a 1D/2D/3D texture or cubemap. So we would create n image views for n images in the chain. And be able to act on them that way.

The function you call to make the view is `vkCreateImageView(device, createInfo, allocator_callback, *VKImageView) -> VkResult`. We should then destroy the image views at the end of the program. I kinda thought they meant in the render loop. Maybe you can do it in the loop too, unless you just want to render once and have that single static image.

Now we need a framebuffer to actually have a render target, for the image view.
