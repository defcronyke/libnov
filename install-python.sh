#!/usr/bin/env bash
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

install_correct_python_version() {
	echo "Installing python."

	SHELL_PROFILE_FILE=$(if [ "$BASH_ENV" == "$HOME/.bashrc" ]; then echo "$HOME/.bash_profile"; else echo "$HOME/.bashrc"; fi)
	PYENV_PROFILE_STR='export PATH="$HOME/.pyenv/bin:$HOME/.pyenv/libexec:$PATH"\n\nif command -v pyenv 1>/dev/null 2>&1; then\n  eval "$(pyenv init -)"\nfi\n'
	
	which pyenv >/dev/null
	if [ $? -ne 0 ]; then
		echo "Installing pyenv."
		curl -sL "https://pyenv.run" | bash
		echo -e "$PYENV_PROFILE_STR" >> "$SHELL_PROFILE_FILE"

		echo "SHELL_PROFILE_FILE=\"$SHELL_PROFILE_FILE\""
		cat "$SHELL_PROFILE_FILE"

		# exec "$SHELL"
		source "$SHELL_PROFILE_FILE"

		ls -al "$HOME/.pyenv/bin"

		ls -al "$HOME/.pyenv/libexec"

		echo "PATH=\"$PATH\""

	else
		echo "Not installing pyenv because it's already installed."
	fi
	
	cat "$SHELL_PROFILE_FILE" | grep "pyenv init -" >/dev/null
	if [ $? -ne 0 ]; then
		echo "Adding pyenv to shell profile: $SHELL_PROFILE_FILE"
		echo -e "$PYENV_PROFILE_STR" >> "$SHELL_PROFILE_FILE"
		# exec "$SHELL"
		source "$SHELL_PROFILE_FILE"
	else
		echo "Not adding pyenv to shell profile because it's already there: $SHELL_PROFILE_FILE"
	fi

	pyenv versions | grep "$PYTHON_FETCH_VERSION" >/dev/null
	if [ $? -ne 0 ]; then
		pyenv install "$PYTHON_FETCH_VERSION"
	fi

	if [ ! -f .python-version ] || [ $(cat .python-version) != "$PYTHON_FETCH_VERSION" ]; then
		pyenv local "$PYTHON_FETCH_VERSION"
	fi

	if [ ! -f ../nov/.python-version ] || [ $(cat ../nov/.python-version) != "$PYTHON_FETCH_VERSION" ]; then
		ln -s "$PWD/.python-version" ../nov/.python-version
	fi

	if [ ! -f ../.python-version ] || [ $(cat ../.python-version) != "$PYTHON_FETCH_VERSION" ]; then
		ln -s "$PWD/.python-version" ../.python-version
	fi

	echo "Checking active python version:"
	pyenv version
}

install_python() {
	python -V >/dev/null
	HAS_PYTHON=$?

	PYTHON_MIN_VERSION="3.9.0"
	PYTHON_FETCH_VERSION=${PYTHON_FETCH_VERSION:-"3.9.1"}
	# PYTHON_SYSTEM_VERSION=$(python -V | cut -d' ' -f2)
	PYTHON_SYSTEM_VERSION="2.9.9"
	PYTHON_VERSION=${PYTHON_VERSION:-$(if [ $HAS_PYTHON -ne 0 ]; then echo "$PYTHON_FETCH_VERSION"; else echo $PYTHON_SYSTEM_VERSION; fi)}

	PYTHON_MIN_V_MAJ=$(echo $PYTHON_MIN_VERSION | cut -d'.' -f1)
	PYTHON_MIN_V_MIN=$(echo $PYTHON_MIN_VERSION | cut -d'.' -f2)
	PYTHON_MIN_V_MIC=$(echo $PYTHON_MIN_VERSION | cut -d'.' -f3)

	PYTHON_V_MAJ=$(echo $PYTHON_VERSION | cut -d'.' -f1)
	PYTHON_V_MIN=$(echo $PYTHON_VERSION | cut -d'.' -f2)
	PYTHON_V_MIC=$(echo $PYTHON_VERSION | cut -d'.' -f3)

	if [[ ("$PYTHON_VERSION" != "$PYTHON_MIN_VERSION") && \
		($PYTHON_V_MAJ -lt $PYTHON_MIN_V_MAJ) || \
		(($PYTHON_V_MAJ -eq $PYTHON_MIN_V_MAJ) && ($PYTHON_V_MIN -lt $PYTHON_MIN_V_MIN)) || \
		(($PYTHON_V_MAJ -eq $PYTHON_MIN_V_MAJ) && ($PYTHON_V_MIN -eq $PYTHON_MIN_V_MIN) && ($PYTHON_V_MIN -le $PYTHON_MIN_V_MIC)) ]]; then

		echo "Warning: The python version on your system is too old. The minimum supported version is \"$PYTHON_MIN_VERSION\"."
		echo "Warning: Fetching version \"$PYTHON_FETCH_VERSION\" and installing it."
		echo "Warning: You can choose to fetch a different version instead by setting the environment variable \"PYTHON_FETCH_VERSION\"."

		set +e
		install_correct_python_version
		res=$?
		set -e

		if [ $res -ne 0 ]; then
			echo "error: Installing python failed."
			return $res
		fi

	else
		echo "The python version on your system is compatible."
	fi
}

install_python $@
