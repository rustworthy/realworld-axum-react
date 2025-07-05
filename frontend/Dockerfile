FROM node:22-alpine AS build

WORKDIR /build

COPY package*.json .
RUN npm ci

COPY . .
RUN npm run build:prod

################################################################################

FROM nginx:alpine

WORKDIR /usr/share/nginx

# copy front-end build from the build step into NGINX static assets root
COPY --from=build /build/build html

CMD ["nginx", "-g", "daemon off;"]