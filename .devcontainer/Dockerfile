FROM 192.168.15.200:80/library/asdf-env:1.5
ARG GIT_USER_NAME

ENV GIT_USER_NAME=${GIT_USER_NAME}\
    ERL_AFLAGS="-kernel shell_history enabled"
RUN git config --global --add safe.directory /ws/asset_texture_render

WORKDIR /ws/asset_texture_render
