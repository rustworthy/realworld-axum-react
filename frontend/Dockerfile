FROM node:22-alpine AS build

ARG BACKEND_URL
ARG REACT_APP_MODE

WORKDIR /build

COPY package*.json .
RUN npm ci

COPY . .

ENV BACKEND_URL=$BACKEND_URL
ENV REACT_APP_MODE=$REACT_APP_MODE
RUN npm run build:prod

################################################################################

FROM nginx:alpine

WORKDIR /usr/share/nginx

# copy front-end build from the build step into NGINX static assets root
COPY --from=build /build/build html
COPY --from=build /build/.kamal/nginx.conf /etc/nginx/conf.d/default.conf

CMD ["nginx", "-g", "daemon off;"]