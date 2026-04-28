FROM python:3.11-bookworm

ENV DEBIAN_FRONTEND=noninteractive
ENV VENV=/opt/venv
ENV CARGO_HOME=/root/.cargo
ENV RUSTUP_HOME=/root/.rustup
ENV PATH="/opt/venv/bin:/root/.cargo/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    make \
    pkg-config \
    texlive-full \
    && rm -rf /var/lib/apt/lists/*

RUN python -m venv .venv_artifact_tches \
    && /opt/venv/bin/pip install --upgrade pip setuptools wheel maturin

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.84.0 \
    && /root/.cargo/bin/rustc --version \
    && /root/.cargo/bin/cargo --version

WORKDIR /artifact

COPY . /artifact

RUN make install

CMD ["bash"]
