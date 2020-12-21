defmodule Day04Ex do
  @moduledoc """
  Documentation for `Day04Ex`.
  """

  def parse_credentials(line) do
    line
    |> String.split()
    |> Enum.map(fn field ->
      case String.split(field, ":") do
        ["byr", v] -> {:byr, v}
        ["iyr", v] -> {:iyr, v}
        ["eyr", v] -> {:eyr, v}
        ["hgt", v] -> {:hgt, v}
        ["hcl", v] -> {:hcl, v}
        ["ecl", v] -> {:ecl, v}
        ["pid", v] -> {:pid, v}
        ["cid", v] -> {:cid, v}
        _ -> {:invalid, field}
      end
    end)
    |> Enum.into(%{})
  end

  def is_valid_passport(%{:byr}), do: :no
  def is_valid_passport(%{}), do: :no
end
