defmodule InputParser do
  def parse() do
    read_input()
    |> Enum.filter(&(&1 == ""))
    |> Enum.reduce(
      {:parse_rules, %{rules: [], my_ticket: "", nearby: []}},
      fn
        {:parse_rules, acc}, "your ticket:" ->
          {:parse_ticket, acc}

        {:parse_rules, acc}, x ->
          {:parse_rules, Map.update(acc, :rules, [x], fn v -> [x | v] end)}

        {:parse_ticket, acc}, x ->
          {:parse_nearby, Map.put(acc, :my_ticket, x)}

        {:parse_nearby, acc}, "nearby tickets:" ->
          {:parse_nearby, acc}

        {:parse_nearby, acc}, x ->
          {:parse_nearby, Map.update(acc, :nearby, [x], fn v -> [x | v] end)}
      end
    )
  end

  def read_input() do
    "input.txt"
    |> File.read!()
    |> String.split("\n")
  end
end
