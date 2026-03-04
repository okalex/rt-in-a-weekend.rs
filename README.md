# Overview

This is a rust implementation of the path tracer from the [Ray Tracing in One Weekend](https://raytracing.github.io/)
series of books. I'm just using this as an opportunity to learn about both path tracing and rust, so the code is
not great, as I'm not a rust programmer and I avoided using any AI tools. Hopefully, it'll get better in time as I do.

The renderer is quite basic at this point (I just finished [the first book](https://raytracing.github.io/books/RayTracingInOneWeekend.html)), though it does support Lambertian, metallic,
and dielectric (glass) materials, as well as depth-of-field and camera positioning. It's currently computed entirely on
the CPU, which is slowwwwwwwww.

## Future planned enhancements

- Complete books [2](https://raytracing.github.io/books/RayTracingTheNextWeek.html) and [3](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
- Implement [BVH optimization](https://en.wikipedia.org/wiki/Bounding_volume_hierarchy)
- Render on the GPU, using [wgpu](https://wgpu.rs/)
- Implement [physically-based rendering](https://en.wikipedia.org/wiki/Physically_based_rendering)
- Support common modeling formats
- Add an interactive mode supporting progressive rendering display and camera movement

## Example render

This is the final image generated after the completion of the first book. It took about seven and a half hours to render at 100 samples per pixel and a max bounce depth of 4 to limit rendering time:

![test_b_100_4_large](https://github.com/user-attachments/assets/0c24c2fe-cdcc-4580-8046-43e016096eca)

Note that you can see the limitations of the low bounce depth by looking at the reflection of the small glass spheres in the large metallic sphere.
