# Use an official Node.js runtime as a parent image
FROM node:20-alpine AS node

# Set a working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json for dependency installation
COPY package*.json ./
COPY server.js ./

# Install application dependencies
RUN npm install

# Copy the rest of the application source code into the container
COPY . .

# Expose the port that your Express application will listen on
EXPOSE 80

# Start the Express application
CMD [ "node", "server.js" ]