# Overview

This is a rust implementation of the path tracer from the [Ray Tracing in One Weekend](https://raytracing.github.io/)
series of books. I'm just using this as an opportunity to learn about both path tracing and rust, so the code is
not great, as I'm not a rust programmer and I avoided using any AI tools. Hopefully, it'll get better in time as I do.

The renderer is quite basic at this point (I just finished [the first book](https://raytracing.github.io/books/RayTracingInOneWeekend.html)), though it does support Lambertian, metallic,
and dielectric (glass) materials, as well as depth-of-field and camera positioning. It's currently computed entirely on
the CPU, which is slowwwwwwwww.

## Future planned enhancements

- ~~Naive multi-threading~~
- ~~Motion blur~~
- Complete books [2](https://raytracing.github.io/books/RayTracingTheNextWeek.html) and [3](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
- ~~Implement [BVH optimization](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy)~~
  - Using camera B and scene B, width = 400px, samples-per-pixel = 20, max depth = 10, multi-threading = true:
  - Before: 537.64s user 9.46s system 766% cpu 1:11.38 total
  - Random axis: 31.25s user 0.40s system 603% cpu 5.241 total
  - Longest axis: 28.51s user 0.34s system 578% cpu 4.987 total
- ~~Texture mapping~~
- ~~Lighting~~
- ~~Volumetrics~~
- GPU-rendering using [wgpu](https://wgpu.rs/)
- [Physically-based rendering](https://en.wikipedia.org/wiki/Physically_based_rendering)
- Support common modeling formats
  - ~~.obj~~
- Normal maps
- Interactive mode
  - ~~Progressive rendering~~
  - Camera movement
  - Update settings
  - Save render
  - Load models
- Importance sampling
- HDR support

## Examples

This is the final image generated after the completion of the first book. It took about 7.5hours to render at 100 samples per pixel and a max bounce depth of 4 to limit rendering time. Note that you can see the limitations of the low bounce depth by looking at the reflection of the small glass spheres in the large metallic sphere. I've since added multi-threading and bounded volume hierarchy (BVH) support and it renders much faster.

![test_b_100_4_large](https://github.com/user-attachments/assets/0c24c2fe-cdcc-4580-8046-43e016096eca)

This is the final image from the second book, which took about 7 hours to render at 5000 samples per pixel and a max bounce depth of 50. It features emissive materials (i.e. lights), volumetrics (fog, smoke, and haze), and texture mapping (color only).

![final_book2_5000_50](https://github.com/user-attachments/assets/3942223d-7f79-406f-8003-4b14143d8f95)

## BVH Implementation results

Using camera B and scene B, width = 400px, samples-per-pixel = 20, max depth = 10, multi-threading = true:
- Before: 537.64s user 9.46s system 766% cpu 1:11.38 total
- After: 31.25s user 0.40s system 603% cpu 5.241 total
