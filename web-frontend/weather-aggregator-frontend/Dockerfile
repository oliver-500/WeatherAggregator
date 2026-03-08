FROM node:25.5.0-alpine3.23 AS build

WORKDIR /app


COPY package*.json ./
RUN npm install


COPY . .
RUN npm run build


# Runtime stage
FROM nginx:alpine

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

COPY weather-aggregator-ca.crt /etc/nginx/certs/weather-aggregator-ca.crt

COPY --from=build /app/dist /usr/share/nginx/html


EXPOSE 8080