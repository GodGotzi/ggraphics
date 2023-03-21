# Rust library with winit wgpu for Desktop Applications - ggprahics
ggprahics is a Rust library that uses WinIt and wgpu to create desktop applications. Unlike other normal libraries, ggprahics has a unique approach of imitating the design of HTML and CSS. This allows users to reuse the majority of their HTML and CSS code and easily convert it into Rust code.

<--
# Installation
To use ggprahics in your Rust project, add the following to your Cargo.toml file:
-->

```
use ggprahics::{Window, Element, ElementType};

fn main() {
    let mut window = Window::new("My Window", 800, 600);
    let h1 = Element::new(ElementType::H1).with_text("Hello, world!");
    let p = Element::new(ElementType::P).with_text("This is a paragraph.");
    window.get_context().add_child(h1);
    window.get_context().add_child(p);
    window.run();
}
```

Yeah Thats what it should look like when its finished, right now nothing works, Project is in early stage!

In this example, we create a Window with a title of "My Window" and dimensions of 800x600 pixels. We then create some Elements to represent the body of the document, an h1 heading, and a p paragraph. We add the heading and paragraph as children of the body, and set the body as the content of the window. Finally, we call run to start the event loop and display the window.

# Contributing
If you'd like to contribute to ggprahics, please feel free to submit a pull request or open an issue on the GitHub repository.

# License
ggprahics is licensed under the MIT License, which is included in the LICENSE file in this repository.
