# Use an official OpenJDK runtime as a parent image
FROM eclipse-temurin:17-jdk-alpine

# Copy the compiled Spring Boot JAR file into the container
COPY target/*.jar app.jar

# Expose the port that your Spring Boot application will listen on
EXPOSE 80

# Start the Spring Boot application
CMD ["java", "-jar", "app.jar"]