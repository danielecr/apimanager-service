FROM node:12.18.3-alpine3.12 as react-build

WORKDIR /app

RUN apk add --no-cache git

ADD gui/env-cmdrc /app/.env-cmdrc

ENV NODE_ENV=production
ENV PUBLIC_URL
ENV API_BASE
ENV API_AUTH
ENV BASE_REALURL
# replace [[PUBLIC_URL]] [[API_BASE]] [[API_AUTH]] [[BASE_REALURL]]

RUN sed -i 's/\[\[PUBLIC_URL\]\]/${PUBLIC_URL}/g' .env-cmdrc
RUN sed -i 's/\[\[API_BASE\]\]/${API_BASE}/g' .env-cmdrc
RUN sed -i 's/\[\[API_AUTH\]\]/${API_AUTH}/g' .env-cmdrc
RUN sed -i 's/\[\[BASE_REALURL\]\]/${BASE_REALURL}/g' .env-cmdrc

RUN git clone https://github.com/landingon-cloud/api-manager-gui.git && \
    cd api-manager-gui && \
    cp ../.env-cmdrc . && \
    npm install --legacy-peer-deps && \
    npm run build

FROM rust:1.70.0 as rust-build
