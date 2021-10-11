# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# Ref. site : https://qiita.com/NA_simple/items/aedaf27f2b8e2ee5187b
RUN apt update && apt -y upgrade
RUN apt install -y sudo && \
    apt install -y build-essential && \
    apt install -y vim && \
    apt install -y wget && \
    apt install -y unzip

RUN cd $HOME && \
    wget https://github.com/Kitware/CMake/releases/download/v3.21.2/cmake-3.21.2.tar.gz && \
    tar zxvf cmake-3.21.2.tar.gz && \
    cd cmake-3.21.2/ && \
    ./bootstrap && \
    make -j12 && sudo make install -j8

RUN echo 'export PATH=$HOME/cmake-3.17.1/bin/:$PATH' >> ~/.bashrc && \
    . ~/.bashrc