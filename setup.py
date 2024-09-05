import os
import sys

from setuptools import setup, find_packages
from setuptools.command.install import install


class CustomInstallCommand(install):
    """
    Custom installation command, set global env var during installation
    """
    def run(self):
        """
        Execute the custom installation command, setting the global var
        """
        abs_path = str(os.getcwd())
        self.set_global_var(abs_path)
        install.run(self)

    def set_global_var(self, abs_path):
        """
        Set a global env var based on the current platform

        Args:
            abs_path (str): the absolute path to be set as part of env var
        """
        var_name = "TRUST_ABS_PATH"
        var_value = f'{abs_path}/sdk'
        if sys.platform.startswith('linux'):
            bashrc_path = os.path.expanduser("~/.bashrc")
            self.update_src_file(bashrc_path, var_name, var_value)
        elif sys.platform == 'darwin':
            zshrc_path = os.path.expanduser("~/.zshrc")
            self.update_src_file(zshrc_path, var_name, var_value)
        else:
            print('[!] platform not supported!')
            sys.exit(1)

    def update_src_file(self, file_path, var_name, var_value):
        """
        Update the specified source file with global env var

        Args:
            file_path (str): path to the shell conf file
            var_name (str): name of the env var
            var_value (str): value to be assigned to the env var
        """
        export_line = f'export {var_name}="{var_value}"\n'
        try:
            with open(file_path, 'r') as s_file:
                lines = s_file.readlines()
        except FileNotFoundError:
            lines = []
        lines = [line for line in lines if not line.startswith(f'export {var_name}=')]
        lines.append(export_line)
        try:
            with open(file_path, 'w') as s_file:
                s_file.writelines(lines)
        except IOError as e:
            print(f'[!] failed to write into {file_path}: {e}')
            sys.exit(1)

setup(
    name='t-rust',
    version='0.0.1',
    description='Simpler yet safer access to the proving ecosystem',
    url='https://safejunction.io',
    packages=find_packages(),
    cmdclass={
        'install': CustomInstallCommand,
    },
    include_package_data=True,
    install_requires=[
        'cbor2',
        'docker'
    ],
    entry_points={
        'console_scripts': [
            't-rust=sdk.cli:main',
        ],
    },
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: MIT License',
        'Operating System :: GNU/Linux, macOS',
    ],
    python_requires='>=3.10',
)
