defmodule ElixirCowboy.Application do
  use Application
  require Logger

  def start(_type, _args) do
    # List all child processes to be supervised
    children = [
      ElixirCowboy.DB,
      {Plug.Cowboy, scheme: :http, plug: ElixirCowboy.Router, options: [port: 80]}
    ]

    Logger.info("Running elixir server...")

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: ElixirCowboy.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
