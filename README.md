# Tharsis
A Vulkan and Rust based controller UI for GNU/Linux.  
It's planed to run as a standalone application atop gamescope to provide a more general controller interface as like
the Steam Deck UI which is tailored towards Steam.  

This is primarily a resaerch project to explore possibilities to mimik a more general purpose controller friendly user interaface. 
This is **not** a full featured desktop environment, window manager or Wayland compositor and not planed to be one. This is what gamescope already provides and will be used for.  

By keepin any window managing out of this project it should also be able to run outside of a gamescope session.

## Requirements
- rustup
- gcc-c++
- ninja
- CMake
- vulkan-devel
- vulkan-tools
- fontconfig-devel

### openSUSE
`sudo zypper in cmake ninja vulkan-devel vulkan-tools gcc-c++ fontconfig-devel`

## Compile and run
`cargo r`