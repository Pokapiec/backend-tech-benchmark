# Use an official OpenJDK runtime as a parent image
FROM openjdk:17-jre-slim AS java

# Set a working directory in the container
WORKDIR /app

# Copy the compiled Spring Boot JAR file into the container
COPY target/your-spring-boot-app.jar /app/app.jar

# Expose the port that your Spring Boot application will listen on
EXPOSE 8080

# Start the Spring Boot application
CMD ["java", "-jar", "app.jar"]