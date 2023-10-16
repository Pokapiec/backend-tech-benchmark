# Use an official .NET Core runtime as a parent image
FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS dotnet

# Set a working directory in the container
WORKDIR /app

# Copy the published .NET Core application into the container
COPY bin/Release/net6.0/publish /app

# Expose the port that your .NET Core application will listen on
EXPOSE 80

# Start the .NET Core application
CMD ["dotnet", "YourApp.dll"]