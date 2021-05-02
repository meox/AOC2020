defmodule Day16Ex do
  @moduledoc """
  Documentation for `Day16Ex`.
  """

  def part1() do
    input = InputParser.parse()
    %{nearby: nearby, rules: rules} = input

    nearby
    |> Enum.flat_map(fn ticket ->
      ticket
      |> Enum.map(fn x -> {x, is_valid(x, rules)} end)
      |> Enum.filter(fn {_, valid} -> valid == false end)
      |> Enum.map(fn {x, _} -> x end)
    end)
    |> Enum.sum()
  end

  def is_valid(field, ruleset) do
    ruleset
    |> Enum.any?(fn {_, ranges} ->
      ranges
      |> Enum.any?(fn {x, y} -> field >= x && field <=y end)
     end)
  end
end
