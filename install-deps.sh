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

pwd="$PWD"

install_libnov_deps() {
	if [ -d "libnov" ]; then
		cd libnov
	fi

  which cargo > /dev/null 2>&1
  HAS_CARGO=$?

  which pacman > /dev/null 2>&1
  HAS_PACMAN=$?
  
  which apt-get > /dev/null 2>&1
  HAS_APT_GET=$?
  
  which dnf > /dev/null 2>&1
  HAS_DNF=$?

  which brew > /dev/null 2>&1
  HAS_brew=$?

  uname -a | grep Darwin
  IS_MACOS=$?

  set -e

  ARCH_DEPS="curl python python-pip base-devel openssl zlib git vulkan-icd-loader lib32-vulkan-icd-loader vulkan-headers cmake"
  DEBIAN_DEPS="make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev python-openssl libx11-dev libvulkan-dev libxcb1-dev xorg-dev cmake"
  FEDORA_DEPS="curl python3 python3-pip make gcc zlib-devel bzip2 bzip2-devel readline-devel sqlite sqlite-devel openssl-devel tk-devel libffi-devel git libX11-devel vulkan cmake"
  MACOS_DEPS="curl python openssl readline sqlite3 xz zlib git cmake"

  if [ $HAS_CARGO -ne 0 ]; then
    echo "error: The Rust programming language toolchain was not detected. Re-run this script after installing the Rust toolchain from here:"
    echo "error: https://rustup.rs"
    return 1
  fi

  if [ $HAS_PACMAN -eq 0 ]; then
    echo "Detected an Arch-like distro. Using pacman."
    sudo pacman -Sy
    sudo pacman --needed -S $ARCH_DEPS
  elif [ $HAS_APT_GET -eq 0 ]; then
    echo "Detected a Debian-like distro. Using apt-get."
    sudo apt-get update
    sudo apt-get install -y lsb-release
    sudo apt-get -t `lsb_release -cs`-backports install -y --no-install-recommends $DEBIAN_DEPS || \
    sudo apt-get install -y --no-install-recommends $DEBIAN_DEPS
  elif [ $HAS_DNF -eq 0 ]; then
    echo "Detected a Fedora-like distro. Using dnf."
    sudo dnf upgrade --refresh
    sudo dnf install $FEDORA_DEPS
  elif [ $IS_MACOS -eq 0 ]; then
      echo "Detected a macOS-like distro. Checking if you have brew installed."
      echo "If you have any trouble, run this script again after following the macOS-specific steps here:"
      echo "https://github.com/gfx-rs/gfx/blob/master/info/getting_started.md#macos-dependencies"
    if [ $HAS_BREW -eq 0 ]; then
      echo "Found brew."
      brew install $MACOS_DEPS
    else
      echo "error: On macOS, you need to install the following dependencies: $MACOS_DEPS"
      echo "error: The easiest way is to re-run this script after installing brew from here: https://brew.sh"
      return 2
    fi
  else
    echo "error: Unknown distro."
    echo "Otherwise, make sure you have Vulkan installed properly and you should be fine."
    return 3
  fi

	./install-python.sh
}

install_libnov_deps $@
exit_code=$?

if [ $exit_code -eq 0 ]; then
	echo "Installing dependencies succeeded."
else
	echo "error: Installing dependencies failed."
fi

cd "$pwd"

exit $exit_code
