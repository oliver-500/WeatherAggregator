FROM node:25.5.0-alpine3.23 AS build

WORKDIR /app


COPY package*.json ./
RUN npm install


COPY . .
RUN npm run build


# Runtime stage
FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html


EXPOSE 80