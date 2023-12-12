# glou
[![docs.rs](https://img.shields.io/docsrs/glou)](https://docs.rs/glou) [![Crates.io](https://img.shields.io/crates/l/glou)](https://crates.io/crates/glou)

A GLSL shader viewer, and debugger, hopefully.

## What to do

- [x] Realtime `uniform` values display.
- [x] Shader compilation error panel.
- [x] A _Live mode_ that hides the UI to make shader fullscreen.
- [x] Support for different `uniform` naming conventions. (`u_time`, `iTime`, etc.)
- [x] Provide a way to reset the `time` uniform at will.
- [ ] Include some GLSL methods documentation and typing, with a simple description and a plot of the function.
- [ ] Provide a way to debug in-GPU variables through some hack or method.
- [ ] Add a screenshot/screencapture panel to allow generating exportable images and videos with the specified size. (Even larger than the current screen for example)
- [x] Support for NewTek NDI to enable sharing shader renders to compliant softwares.
- [x] Find & fix the memory leaks in the canvas OpenGL code.
- [ ] Investigate `rfd` prompt memory increases
