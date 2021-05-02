defmodule InputParser do
  def parse(file_name) do
    {_, result} =
      file_name
      |> read_input()
      |> Enum.filter(&(&1 != ""))
      |> Enum.reduce(
        {:parse_rules, %{rules: %{}, my_ticket: "", nearby: []}},
        fn
          "your ticket:", {:parse_rules, acc} ->
            {:parse_ticket, acc}

          x, {:parse_rules, acc} ->
            {name, range} = parse_rule(x)

            {:parse_rules,
             Map.update(acc, :rules, %{name => range}, fn v -> Map.put(v, name, range) end)}

          x, {:parse_ticket, acc} ->
            {:parse_nearby, Map.put(acc, :my_ticket, parse_ticket(x))}

          "nearby tickets:", {:parse_nearby, acc} ->
            {:parse_nearby, acc}

          x, {:parse_nearby, acc} ->
            parsed_ticket = parse_ticket(x)

            {:parse_nearby,
             Map.update(acc, :nearby, [parsed_ticket], fn v -> [parsed_ticket | v] end)}
        end
      )

    result
  end

  defp read_input(file_name) do
    file_name
    |> File.read!()
    |> String.split("\n")
  end

  # "zone: 39-786 or 807-969"
  @spec parse_rule(String.t()) :: any()
  defp parse_rule(rule) do
    [name, vs] =
      rule
      |> String.split(":")
      |> Enum.map(&String.trim(&1))

    ranges =
      vs
      |> String.split("or")
      |> Enum.map(&String.trim(&1))
      |> Enum.map(fn r ->
        [min, max] =
          r
          |> String.split("-")
          |> Enum.map(&Integer.parse(&1))
          |> Enum.filter(fn
            {_num, ""} -> true
            _ -> false
          end)
          |> Enum.map(fn {num, _} -> num end)

        {min, max}
      end)

    {name, ranges}
  end

  defp parse_ticket(ticket) do
    ticket
    |> String.split(",")
    |> Enum.map(&Integer.parse(&1))
    |> Enum.filter(fn
      {_num, ""} -> true
      _ -> false
    end)
    |> Enum.map(fn {num, _} -> num end)
  end
end
