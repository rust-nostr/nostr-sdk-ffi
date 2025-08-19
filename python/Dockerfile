FROM python:3.9-slim

# Set up working directory
WORKDIR /build

# Create directories for mounting volumes
RUN mkdir -p python binding binaries dist

# Copy source
COPY src/nostr-sdk/__init__.py python/src/nostr-sdk/
COPY LICENSE python/
COPY MANIFEST.in python/
COPY pyproject.toml python/
COPY README.md python/
COPY requirements.txt python/
COPY setup.py python/

# Copy the build script
COPY buildwheel.sh .
RUN chmod +x buildwheel.sh

# Install Python build tools
RUN pip install -r python/requirements.txt

# Update permissions
RUN chmod -R 777 /build

# Set the entrypoint to our build script
ENTRYPOINT ["/build/buildwheel.sh"]
