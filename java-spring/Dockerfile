# Stage 1: Build the application using Maven
FROM maven:3.8.4-openjdk-17 as builder

# Set the working directory in the Docker image
WORKDIR /app

# Copy the Maven project files and download dependencies to take advantage of cached layers
COPY pom.xml .
RUN mvn dependency:go-offline

# Copy the rest of your project's files
COPY src ./src

# Package the application
RUN mvn package -DskipTests

# Stage 2: Create the runtime image
FROM openjdk:17-slim

# Set the working directory in the Docker image
WORKDIR /app

# Copy the built artifact from the builder stage
COPY --from=builder /app/target/*.jar app.jar

# Expose the port the app runs on
EXPOSE 80

# Run the jar file 
ENTRYPOINT ["java", "-jar", "/app/app.jar"]