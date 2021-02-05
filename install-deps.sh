#!/bin/bash
# Copyright (c) 2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# All uses of this project in part or in whole are governed
# by the terms of the license contained in the file titled
# "LICENSE" that's distributed along with the project, which
# can be found in the top-level directory of this project.
#
# If you don't agree to follow those terms or you won't
# follow them, you are not allowed to use this project or
# anything that's made with parts of it at all. The project
# is also	depending on some third-party technologies, and
# some of those are governed by their own separate licenses,
# so furthermore, whenever legally possible, all license
# terms from all of the different technologies apply, with
# this project's license terms taking first priority.

install_libnov_deps() {
  which pacman > /dev/null 2>&1
  HAS_PACMAN=$?
  
  which apt-get > /dev/null 2>&1
  HAS_APT_GET=$?
  
  which dnf > /dev/null 2>&1
  HAS_DNF=$?

  set -e

  ARCH_DEPS="vulkan-icd-loader lib32-vulkan-icd-loader vulkan-headers cmake"
  DEBIAN_DEPS="libx11-dev libvulkan-dev libxcb1-dev xorg-dev cmake"
  FEDORA_DEPS="libX11-devel vulkan cmake"

  if [ $HAS_PACMAN -eq 0 ]; then
    echo "Detected an Arch-like distro. Using pacman."
    sudo pacman -Syy
    sudo pacman -S $ARCH_DEPS
  elif [ $HAS_APT_GET -eq 0 ]; then
    echo "Detected a Debian-like distro. Using apt-get."
    sudo apt-get update
    sudo apt-get install $DEBIAN_DEPS
  elif [ $HAS_DNF -eq 0 ]; then
    echo "Detected a Fedora-like distro. Using dnf."
    sudo dnf upgrade --refresh
    sudo dnf install $FEDORA_DEPS
  else
    echo "error: Unknown distro. If you're on MacOS, see here:"
    echo "https://github.com/gfx-rs/gfx/blob/master/info/getting_started.md#macos-dependencies"
    echo ""
    echo "Otherwise, make sure you have Vulkan installed properly and you should be fine."
    return 1
  fi
}

install_libnov_deps $@
