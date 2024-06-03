import Config

config :simple_server, ElixirCowboy.DB,
  database: "postgres",
  username: "postgres",
  password: "postgres",
  hostname: "postgres",
  pool_size: 10
