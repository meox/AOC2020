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
        ["byr", v] -> {:byr, Integer.parse(v)}
        ["iyr", v] -> {:iyr, Integer.parse(v)}
        ["eyr", v] -> {:eyr, Integer.parse(v)}
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

  def is_valid_passport(
        %{byr: _, iyr: _, eyr: _, hgt: _, hcl: _, ecl: _, pid: _, cid: _} = passport
      ) do
    validate_fields(passport)
  end

  def is_valid_passport(%{byr: _, iyr: _, eyr: _, hgt: _, hcl: _, ecl: _, pid: _} = passport) do
    validate_fields(passport)
  end

  def is_valid_passport(%{}), do: false

  def validate_fields(%{byr: byr, iyr: iyr, eyr: eyr, hgt: hgt, hcl: hcl, ecl: ecl, pid: pid}) do
    valid_birth(byr) &&
      valid_year(iyr) &&
      valid_expir(eyr) &&
      valid_height(hgt) &&
      valid_hair_color(hcl) &&
      valid_eye_color(ecl) &&
      valid_pid(pid)
  end

  def valid_birth(:error), do: false
  def valid_birth({n, ""}), do: n >= 1920 && n <= 2002
  def valid_birth(_), do: false

  def valid_year(:error), do: false
  def valid_year({n, ""}), do: n >= 2010 && n <= 2020
  def valid_year(_), do: false

  def valid_expir(:error), do: false
  def valid_expir({n, ""}), do: n >= 2020 && n <= 2030
  def valid_expir(_), do: false

  def valid_height(hgt) do
    with %{"v" => v, "u" => u} <- Regex.named_captures(~r/^(?<v>\d+)(?<u>cm|in)$/, hgt),
         {x, ""} <- Integer.parse(v) do
      case u do
        "cm" -> x >= 150 && x <= 193
        "in" -> x >= 59 && x <= 76
        _ -> false
      end
    else
      _ -> false
    end
  end

  def valid_hair_color(hcl) do
    Regex.match?(~r/^#[0-9,a-f]{6}$/, hcl)
  end

  def valid_pid(pid) do
    Regex.match?(~r/^[0-9]{9}$/, pid)
  end

  def valid_eye_color("amb"), do: true
  def valid_eye_color("blu"), do: true
  def valid_eye_color("brn"), do: true
  def valid_eye_color("gry"), do: true
  def valid_eye_color("grn"), do: true
  def valid_eye_color("hzl"), do: true
  def valid_eye_color("oth"), do: true
  def valid_eye_color(_), do: false

  def load_input() do
    case File.read("./data/input.txt") do
      {:ok, body} ->
        body
        |> String.split("\n\n")
        |> Enum.map(fn ls -> ls |> String.replace("\n", " ") end)

      {:error, reason} ->
        {:error, reason}
    end
  end

  def count_valid_passports(passports) do
    passports
    |> Enum.map(&parse_credentials/1)
    |> Enum.filter(&is_valid_passport/1)
    |> Enum.count()
  end
end
