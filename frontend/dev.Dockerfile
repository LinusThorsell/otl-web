FROM node:22-alpine3.19

WORKDIR /app

COPY ./ ./

RUN npm install -g npm@latest
RUN npm install

EXPOSE 3000

