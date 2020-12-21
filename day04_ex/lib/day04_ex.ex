defmodule Day04Ex do
  @moduledoc """
  Documentation for `Day04Ex`.
  """
  @doc """
  Parse a credentials

  ## Examples

    iex> Day04Ex.parse_credentials("byr:1937 iyr:2017 cid:147 hgt:183cm")
    %{byr: "1937", iyr: "2017", cid: "147", hgt: "183cm"}

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

  def is_valid_passport(%{byr: _, iyr: _, eyr: _, hgt: _, hcl: _, ecl: _, pid: _, cid: _}), do: true
  def is_valid_passport(%{byr: _, iyr: _, eyr: _, hgt: _, hcl: _, ecl: _, pid: _}), do: true
  def is_valid_passport(%{}), do: false

  def load_input() do
    case File.read("./data/input.txt") do
      {:ok, body}      ->
        body
        |> String.split("\n\n")
        |> Enum.map(fn ls -> ls |> String.replace("\n", " ") end)
      {:error, reason} -> {:error, reason}
    end
  end

  def count_valid_passports(passports) do
    passports
    |> Enum.map(&parse_credentials/1)
    |> Enum.filter(&is_valid_passport/1)
    |> Enum.count()
  end
end
