from setuptools import setup, find_packages
import subprocess
import os

setup(
    name="openiap",
    version="0.0.38",
    author="OpenIAP ApS / Allan Zimmerman",
    author_email="info@openiap.io",
    description="Simple openiap api wrapper using proto",
    long_description=open('README.md').read(),
    long_description_content_type="text/markdown",
    url="https://github.com/openiap/pyapi",
    packages=find_packages(),
    include_package_data=True,
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Mozilla Public License 2.0 (MPL 2.0)",
        "Operating System :: OS Independent",
    ],
    package_data={
        'openiap': [
            'lib/*',
            ]
    },
)

lib_dir = os.path.join(os.path.dirname(__file__), 'openiap', 'lib')
os.makedirs(lib_dir, exist_ok=True)
try:
    subprocess.run(['openiap-bootstrap', '--dir', lib_dir], check=False)
except Exception:
    pass
