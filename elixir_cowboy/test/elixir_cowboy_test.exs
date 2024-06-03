defmodule ElixirCowboyTest do
  use ExUnit.Case
  doctest ElixirCowboy

  test "greets the world" do
    assert ElixirCowboy.hello() == :world
  end
end
