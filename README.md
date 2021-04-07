<img src="assets/banner.png" alt="Scion" />

Scion is a 2D game library made in rust. 

> Please note that this project is in its first milestones and is subject to change according to convience needs and big features coming.
> You can use Scion as you want although I just made this open source to serve the community not to be a competitor to the current Rust game engine eco-system.

## Why this project ? 

Well, firstly because it' a good way to learn.

Then because a lot of projects these days focus on adding a lot of feature pretexting feature parity with big editors or game engines. 
Here I focus on the features that I really need for my projects. I won't add things just because they are cool, but because I need them in 
a game project

Scion relies on a short list of principles that also serves as a guideline.

### Goals

- Strong focus on **2D** only.
- **Easy** and **Fun** to use.
- Clean and readable source code.
- Tiled integration

### Non goals

- Ultra/over optimized code and performances. For this, please try other engines or build your own !
- 3D
- Editor

## Why ECS ?

Today, ECS is like a 'magic' word for games, game libs and game engine. But ECS is not and must not be considered as a universal answer to multiple questions.
I believe that ECS has its strength and its weaknesses. 
The choice of it here is made because the main target games of this lib at its start were to be able to make : 
- a complex city building game.
- a pkmn fan game with real time trainer progression and wild pkmn.
- granular network packets representation && dot simulation games. 

## Notable dependencies

These are the dependencies this project is relying on. Thanks to these awesome communities, we are able to build this kind of tiny projects. 

- <a href="https://github.com/rust-windowing/winit" target="blank">winit</a> and <a href="https://github.com/gfx-rs/wgpu-rs" target="blank">wgpu</a> (windowing and multi backend rendering)
- <a href="https://github.com/amethyst/legion" target="blank">legion</a> (Entity component system)
- <a href="https://github.com/termhn/ultraviolet" target="blank">ultraviolet</a> (Maths)

