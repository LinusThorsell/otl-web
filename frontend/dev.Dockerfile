FROM node:22-alpine3.19

WORKDIR /app

RUN npm install -g npm@latest

COPY ./ ./

EXPOSE 3000

