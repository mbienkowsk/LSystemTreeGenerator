## LLM generated ahh architecture overview

| Module        | Owns/Manages                   | Converts/Produces                                                          | Used By       | Key Responsibility                                                                          |
|---------------|--------------------------------|----------------------------------------------------------------------------|---------------|---------------------------------------------------------------------------------------------|
| App           | Renderer, FlyCamera            | Input → Camera/Render actions, GUI inputs → fractal transformations        | Main          | Runs event loop, handles mouse/keyboard input, handles gui input                            |
| Renderer      | Window, Display, GuiController | Geometry/Matrices → Draw calls                                             | App           | Draws 3D objects, manages shaders                                                           |
| Camera        | Camera state                   | Input → View/Projection matrices                                           | App, Renderer | Controls camera movement/view                                                               |
| Shaders       | Shader sources                 | GLSL code                                                                  | Renderer      | Defines rendering logic                                                                     |
| Main          | App                            | -                                                                          | -             | Starts the application                                                                      |
| GuiController | EguiGlium, config options      | GUI inputs → config                                                        | Renderer      | Configuring render and L-system                                                             |
| LSystem       | -                              | Axiom, production rules → string definition of a fractal (turtle commands) | App           | Applying grammatical production rules                                                       |
| Model loader  | -                              | .obj file path → tobj::Model with geometry data                            | App           | Loading .obj 3D models                                                                      |
| Turtle        | -                              | String definition of a fractal → series of transformation matrices         | App           | Parsing turtle instructions (fractal definition) into useful format for rendering 3D object |


**Data Flow:**  
- `App`
  - receives input, updates `Camera`.
  - fetches `LSystemConfig` from `GuiController` and checks for changes.
  - computes transform matrices using `TurtleInterpreter` and passes them to.
  - loads 3D models using `ModelLoader`.
  - passes `Camera` parameters, 3D models, and computed fractal transform matrices to `Renderer`.
- `Renderer`
  - uses `Camera` matrices and `Shaders` to render objects.
  - uses `GuiController` to draw GUI on top of the 3D scene.
- `Shaders` define how objects are drawn.
