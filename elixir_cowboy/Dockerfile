# Use the official Elixir image
FROM elixir:1.14.4-alpine

# Set environment variables
ENV MIX_ENV=prod
ENV LANG=C.UTF-8

# Install Hex and Rebar
RUN mix local.hex --force && \
    mix local.rebar --force

# Install Postgres client
RUN apk add --no-cache postgresql-client

# Create and set the working directory
WORKDIR /app

# Copy the mix.exs and mix.lock files
COPY mix.exs mix.lock ./

# Install dependencies
RUN mix deps.get --only prod

# Copy the rest of the application code
COPY . .

# Compile the application
RUN mix do deps.compile, compile

# Expose the port on which the app runs
EXPOSE 80

# Run the application
CMD ["mix", "run", "--no-halt"]