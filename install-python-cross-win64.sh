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

install_python_cross_win64() {
  echo "Installing win64 version of Python for cross-compiling."

  cd ../nov

  mkdir -p python-cross-win64
  cd python-cross-win64
  
  if [ ! -d "Python-3.9.1" ]; then
    wget "https://www.python.org/ftp/python/3.9.1/Python-3.9.1.tar.xz"
    wget "https://www.python.org/ftp/python/3.9.1/python-3.9.1-embed-amd64.zip"
    tar xf Python-3.9.1.tar.xz
    unzip python-3.9.1-embed-amd64.zip
    cp python39.dll python3.9.dll
  fi

  mkdir -p ../target/x86_64-pc-windows-gnu/debug/
  mkdir -p ../target/x86_64-pc-windows-gnu/release/

  cp python39.dll python3.9.dll ../target/x86_64-pc-windows-gnu/debug/
  cp python39.dll python3.9.dll ../target/x86_64-pc-windows-gnu/release/
  
  cd ../../libnov
}

install_python_cross_win64 $@
