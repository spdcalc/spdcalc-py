# Variables
pyenv_name := ".myenv"
requirements_file := "py-examples/requirements.txt"

# Default task
default: run_example

clean:
  rm -rf {{pyenv_name}}
  echo "Virtual environment removed."

# Check if pyenv is installed
check_virtualenv_installed:
  @if ! command -v virtualenv &> /dev/null; then \
    echo "virtualenv is not installed. Please install virtualenv first."; \
    exit 1; \
  fi

# Create virtual environment
create_virtualenv: check_virtualenv_installed
  virtualenv {{pyenv_name}}
  echo "Virtual environment created."

# Install requirements
install_requirements: create_virtualenv
  {{pyenv_name}}/bin/pip install -r {{requirements_file}}
  echo "Requirements installed."

run_maturin: install_requirements
  source {{pyenv_name}}/bin/activate && \
  maturin develop

# Run the example
run_example: run_maturin
  source {{pyenv_name}}/bin/activate && \
  {{pyenv_name}}/bin/python3 py-examples/simple.py
