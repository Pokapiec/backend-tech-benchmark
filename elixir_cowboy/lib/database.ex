defmodule ElixirCowboy.DB do
  use Ecto.Repo,
    otp_app: :simple_server,
    adapter: Ecto.Adapters.Postgres

  def query(sql) do
    case Ecto.Adapters.SQL.query(__MODULE__, sql, []) do
      {:ok, result} -> result.rows
      {:error, _reason} -> []
    end
  end
end
