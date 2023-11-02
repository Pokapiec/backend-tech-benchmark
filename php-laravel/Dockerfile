# Use the official PHP 8.1 image with Apache
FROM php:8.1-apache

# Set the working directory in the container
WORKDIR /var/www/html

# Copy your Laravel application to the container
COPY . .

# Install system dependencies and PHP extensions
RUN apt-get update && \
    apt-get install -y libpng-dev libjpeg-dev zip && \
    docker-php-ext-configure gd --with-jpeg && \
    docker-php-ext-install gd pdo pdo_mysql && \
    a2enmod rewrite

# Expose port 80
EXPOSE 80

# Configure Apache for Laravel
COPY laravel.conf /etc/apache2/sites-available/000-default.conf

# Enable Apache modules and restart the web server
RUN a2enmod rewrite
RUN service apache2 restart

# Set up Laravel environment
ENV APP_KEY=your-secret-key
ENV APP_DEBUG=false
ENV APP_URL=http://0.0.0.0:80

# Set the Laravel storage directory permissions
RUN chmod -R 777 storage bootstrap/cache

# Install Composer
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

# Install Laravel dependencies
RUN composer install

# Start the Apache web server
CMD ["apache2-foreground"]