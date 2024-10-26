from setuptools import setup, find_packages

setup(
    name="openiap",
    version="0.0.11",
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
