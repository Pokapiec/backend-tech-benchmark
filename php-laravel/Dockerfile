# Use an official PHP runtime as a parent image
FROM php:8.1-fpm AS php

# Set working directory in the container
WORKDIR /var/www

# Install system dependencies and PHP extensions
RUN apt-get update && apt-get install -y \
    git \
    libpq-dev \
    && docker-php-ext-install pdo pdo_pgsql

# Install Composer
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

# Copy the application source code into the container
COPY . /var/www

# Install application dependencies using Composer
RUN composer install --no-interaction

# Use a Node.js parent image for frontend assets (if applicable)
# For example, if you have JavaScript build tools like npm or yarn
# You can use a node image like this:
# FROM node:16 AS node
# COPY package.json package-lock.json /var/www/
# RUN npm install

# Build frontend assets (if applicable)
# RUN npm run production

# Use a lightweight nginx image for serving the application
FROM nginx:latest

# Copy Laravel application from the php container
COPY --from=php /var/www /var/www

# Copy nginx configuration file
COPY ./docker/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80
EXPOSE 80

# Start nginx in the foreground
CMD ["nginx", "-g", "daemon off;"]