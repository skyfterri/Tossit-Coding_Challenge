# Stage 1: Build the Angular application
# We use a Node.js image as our build environment because Angular applications are built with Node.js tools.
FROM node:18-alpine AS build

# Set the working directory inside the container. All subsequent commands will run from here.
WORKDIR /app

# Copy package.json and package-lock.json first.
# This allows Docker to cache the 'npm install' step. If only source code changes,
# npm install won't need to re-run, speeding up builds.
COPY package*.json ./

# Install project dependencies.
RUN npm install

# Copy the rest of the application source code into the container.
COPY . .

# Build the Angular application for production.
# --output-path ./dist/my-responsive-app: Specifies where the compiled output will be placed.
#   Make sure 'my-responsive-app' matches your actual project name as it appears in the dist folder.
# --configuration production: Applies production optimizations like minification, tree-shaking, etc.
RUN npm run build -- --output-path ./dist/my-responsive-app --configuration production

# Stage 2: Serve the application with Nginx
# We use a lightweight Nginx image to serve the static files produced by the build stage.
# Nginx is an efficient web server for static content.
FROM nginx:alpine

# Copy the built Angular application from the 'build' stage to Nginx's default web root.
# The Nginx default web root is /usr/share/nginx/html.
COPY --from=build /app/dist/Product-App /usr/share/nginx/html

# Copy a custom Nginx configuration file.
# This is crucial for Single-Page Applications (SPAs) like Angular, as it ensures
# that all routes are handled by index.html, which is necessary for Angular's client-side routing.
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80. This tells Docker that the container listens on port 80 at runtime.
# When running the container, you will map a host port to this container port (e.g., -p 8080:80).
EXPOSE 80

# Command to run Nginx when the container starts.
# "-g 'daemon off;'" keeps Nginx running in the foreground, which is required for Docker containers.
CMD ["nginx", "-g", "daemon off;"]