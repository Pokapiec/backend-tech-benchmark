# Use an official Node.js runtime as a parent image
FROM node:16 AS node

# Set a working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json for dependency installation
COPY package*.json ./

# Install application dependencies
RUN npm install

# Copy the rest of the application source code into the container
COPY . .

# Expose the port that your Express application will listen on
EXPOSE 3000

# Start the Express application
CMD [ "node", "app.js" ]