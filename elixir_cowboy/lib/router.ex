defmodule ElixirCowboy.Router do
  use Plug.Router

  plug :match
  plug :dispatch

  get "/string/" do
    send_resp(conn, 200, "Hello world!")
  end

  get "/simple-json/" do
    send_resp(conn, 200, Jason.encode!(%{
      key1: "value1",
      key2: "value2",
      key3: "value3",
      key_nest: %{
        kn1: "value_nest_1",
        knn2: %{key: "value"}
      },
    }))
  end

  get "/query-params/" do
    conn = Plug.Conn.fetch_query_params(conn)
    params = conn.query_params
    send_resp(conn, 200, Jason.encode!(params))
  end

  post "/file-upload/" do
    {:ok, body, conn} = Plug.Conn.read_body(conn)
    send_resp(conn, 200, body)
  end

  get "/sql-select/" do
    result = ElixirCowboy.DB.query("SELECT * FROM public.exampletable ORDER BY id ASC")
    data = Enum.map(result, fn x -> %{
      id: Enum.at(x, 0),
      firstname: Enum.at(x, 1),
      lastname: Enum.at(x, 2),
      age: Enum.at(x, 3),
      salary: Enum.at(x, 4),
      birtdate: Enum.at(x, 5),
      isactive: Enum.at(x, 6),
      email: Enum.at(x, 7),
      phonenumber: Enum.at(x, 8),
      adress: Enum.at(x, 9),
    } end)
    send_resp(conn, 200, Jason.encode!(data))
  end

  match _ do
    send_resp(conn, 404, "Not found")
  end

end
