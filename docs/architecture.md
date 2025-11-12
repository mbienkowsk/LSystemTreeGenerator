## LLM generated ahh architecture overview

| Module    | Owns/Manages           | Converts/Produces                | Used By         | Key Responsibility                |
|-----------|------------------------|----------------------------------|-----------------|-----------------------------------|
| App       | Renderer, FlyCamera    | Input → Camera/Render actions    | Main            | Runs event loop, handles input    |
| Renderer  | Window, Display        | Geometry/Matrices → Draw calls   | App             | Draws 3D objects, manages shaders |
| Camera    | Camera state           | Input → View/Projection matrices | App, Renderer   | Controls camera movement/view     |
| Shaders   | Shader sources         | GLSL code                        | Renderer        | Defines rendering logic           |
| Main      | App                    | -                                | -               | Starts the application            |


**Data Flow:**  
- `App` receives input, updates `Camera`, and tells `Renderer` to draw.
- `Renderer` uses `Camera` matrices and `Shaders` to render objects.
- `Shaders` define how objects are drawn.
