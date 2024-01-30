# Glint ✨ - A Rust Pathtracer

![output](https://github.com/Sullym8/glint/assets/48613444/fd78f8a8-f72b-416e-a6c7-58fd79cd730f)
![finl](https://github.com/Sullym8/glint/assets/48613444/342190df-8239-4daf-8b68-3f3bea71c3b7)

Glint is a hobby Pathtracer written in Rust that implements a few modern features. I built this over Winter 2023 to learn the Rust Language, and it should evolve as my understanding of Rust grows :)

## Features
- Wavefront rendering
  - Support for .obj and .mtl files
- Bounding Volume Hierarchy (BVH) Acceleration
  - Axis Aligned Bounding Boxes (AABB)  
  - Midpoint Heuristic
- Mutlithreaded CPU Rendering 
- Smooth shading (Gouraud)   
- PBR Materials (also a few debug materials)
  - Diffuse
  - Glossy
  - Glass
  - Metallic
  - Emissive
- Positionable and configurable Camera
  - LookFrom and LookAt
  - FOV
  - Image Dimensions
- Sphere and Triangle primitive types

### Todo
- [ ] Camera Depth of Field
- [ ] Object Transformations
- [ ] Import scene from YAML/TOML
- [ ] A better Glossy Shader
- [ ] Lots of cleanup 😅

## Screenshots
![stylised](https://github.com/Sullym8/glint/assets/48613444/bcc2b28a-4fde-4fab-beca-e6ee89901bac)
![glass_trooper](https://github.com/Sullym8/glint/assets/48613444/81434c1b-3236-4a50-b2ad-cbc49161074d)
![lightball](https://github.com/Sullym8/glint/assets/48613444/43c57f3d-c77c-4e56-b027-b7a500b189c1)


## Credits
Libraries used in this project:
- Rayon: https://docs.rs/rayon/latest/rayon/
- Tiny OBJ Loader: https://docs.rs/tobj/latest/tobj/

Resources:
- [Raytracing in One Weekend Series](https://raytracing.github.io/)
- [Scratchapixel 3.0](https://www.scratchapixel.com/index.html)
- [Ray Tracing Playlist by The Cherno](https://www.youtube.com/watch?v=gfW1Fhd9u9Q&list=PLlrATfBNZ98edc5GshdBtREv5asFW3yXl)
- [Spatial Acceleration Structures by TU Wien](https://www.youtube.com/watch?v=MzUxOe5x24w&t=2742s)



<!--![raytracer_first](https://github.com/Sullym8/glint/assets/48613444/ea7e0124-ac88-4cb5-a0a5-716f7a1766f4)

![raytracer_2balls](https://github.com/Sullym8/glint/assets/48613444/b872e89b-83a2-45fc-8104-ddf80284deaf)

![raytracer_diffuse](https://github.com/Sullym8/glint/assets/48613444/d071d696-40c5-4b86-a641-244d396351aa)

![raytracer_metal_diffuse_test](https://github.com/Sullym8/glint/assets/48613444/61254be2-00af-40a9-bd12-2d8538e2f17a)

![raytracer_glass_2](https://github.com/Sullym8/glint/assets/48613444/0798df25-76ed-4f0a-b66d-9442e264fa4c)

![output](https://github.com/Sullym8/glint/assets/48613444/4b3fb86f-9deb-43f4-91a9-7f55163a7e5f)

![output](https://github.com/Sullym8/glint/assets/48613444/a26a48be-85b3-4794-8e00-eaa57a7cc47f)

![output](https://github.com/Sullym8/glint/assets/48613444/f8822725-3c9f-42eb-b465-c7c2add56cfb)

![glint](https://github.com/Sullym8/glint/assets/48613444/6bd6fbd5-fe4c-4ad7-a353-2cdc7733457c)

![light](https://github.com/Sullym8/glint/assets/48613444/00f5d41a-978d-47ee-83fe-9bbd3aca50e2)

![output](https://github.com/Sullym8/glint/assets/48613444/d0868589-418f-4c79-bc89-ab8651cab03c)

![output](https://github.com/Sullym8/glint/assets/48613444/be8a5595-c658-4c8a-b661-1d37e753a3ad)

![output](https://github.com/Sullym8/glint/assets/48613444/be6f58f1-80d1-4e63-a313-02392d51eded)

![stanford_bunny](https://github.com/Sullym8/glint/assets/48613444/fb0c64c2-cf9a-4162-b554-a56709bc7c11)

![stylised](https://github.com/Sullym8/glint/assets/48613444/89365e6d-d931-491f-95fe-a07cba7d244d)

![output](https://github.com/Sullym8/glint/assets/48613444/513614ff-eccc-4263-bbce-65398ba1b053)-->


