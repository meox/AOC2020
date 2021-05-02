defmodule Day16Ex do
  @moduledoc """
  Documentation for `Day16Ex`.
  """

  def part1() do
    input = InputParser.parse("input.txt")
    %{nearby: nearby, rules: rules} = input

    nearby
    |> Enum.flat_map(fn ticket ->
      ticket
      |> Enum.map(fn x -> {x, valid?(x, rules)} end)
      |> Enum.filter(fn {_, valid} -> valid == false end)
      |> Enum.map(fn {x, _} -> x end)
    end)
    |> Enum.sum()
  end

  def part2() do
    %{nearby: nearby, rules: rules, my_ticket: my_ticket} =
      "input.txt"
      |> InputParser.parse()
      |> remove_invalid_ticket()

    nearby_by_col =
      nearby
      |> Enum.reduce(%{}, fn ticket, acc ->
        ticket
        |> map_ticket()
        |> merge_mapped_ticket(acc)
      end)

    resolved_ticket =
      nearby_by_col
      |> Enum.map(fn {k, vs} -> {k, find_common_rules(vs, rules)} end)
      |> by_type()
      |> reducer()
      |> Enum.map(fn {k, [v]} -> {k, v} end)
      |> Enum.reduce(%{}, fn {k, pos}, acc -> Map.put(acc, pos, k) end)
      |> resolve_ticket(my_ticket)

    resolved_ticket
    |> Enum.filter(fn {k, _v} -> String.starts_with?(k, "departure") end)
    |> Enum.map(fn {_k, v} -> v end)
    |> Enum.reduce(fn x, acc -> x * acc end)
  end

  def resolve_ticket(resolver, ticket) do
    ticket
    |> Stream.with_index()
    |> Stream.map(fn {v, idx} -> {Map.get(resolver, idx), v} end)
    |> Enum.into(%{})
  end

  def by_type(ms) do
    ms
    |> Enum.reduce(%{}, fn {k, set}, acc ->
      set
      |> MapSet.to_list()
      |> Enum.reduce(
        acc,
        fn rule_name, acc ->
          Map.update(acc, rule_name, [k], fn vs -> [k | vs] end)
        end
      )
    end)
  end

  def reducer(ps) do
    ps
    |> Enum.filter(fn {_k, vs} -> length(vs) == 1 end)
    |> continue_reduce(ps)
  end

  defp continue_reduce(uniques, ps) when length(uniques) == length(ps), do: ps

  defp continue_reduce(uniques, ps) do
    ps
    |> Enum.map(fn
      {rule_name, vs} when length(vs) == 1 -> {rule_name, vs}
      {rule_name, vs} -> {rule_name, filter_out(vs, uniques)}
    end)
    |> reducer()
  end

  def filter_out(vs, uniques) do
    vs
    |> Enum.filter(fn x -> !in_uniques_map(x, uniques) end)
  end

  def in_uniques_map(x, uniques) do
    uniques
    |> Enum.map(fn {_, [v]} -> x == v end)
    |> Enum.any?()
  end

  def find_common_rules(xs, rules) do
    xs
    |> Enum.map(fn x -> valid_rules(x, rules) end)
    |> Enum.reduce(fn x, acc -> MapSet.intersection(acc, x) end)
  end

  def valid_rules(field, rules) do
    rules
    |> Enum.filter(fn {_, ranges} ->
      Enum.any?(ranges, fn {min, max} -> field >= min && field <= max end)
    end)
    |> Enum.map(fn {k, _} -> k end)
    |> MapSet.new()
  end

  # take a ticket (a list of num and retur a map[colum_index] -> value)
  # ex: [123,145,11] -> %{0: 123, 1: 145, 2: 11}
  def map_ticket(ticket) do
    ticket
    |> Stream.with_index()
    |> Enum.reduce(%{}, fn {v, idx}, acc -> Map.put(acc, idx, [v]) end)
  end

  def merge_mapped_ticket(ticket_a, ticket_b) do
    ticket_a
    |> Enum.reduce(ticket_b, fn {k, v}, acc -> Map.update(acc, k, v, fn vs -> v ++ vs end) end)
  end

  def remove_invalid_ticket(input) do
    %{nearby: nearby, rules: rules} = input

    new_nearby =
      nearby
      |> Enum.filter(fn ticket -> valid_ticket?(ticket, rules) end)

    %{input | nearby: new_nearby}
  end

  def valid_ticket?(ticket, rules) do
    ticket
    |> Enum.all?(fn x -> valid?(x, rules) end)
  end

  def valid?(field, rules) do
    rules
    |> Enum.any?(fn {_, ranges} ->
      ranges
      |> Enum.any?(fn {x, y} -> field >= x && field <= y end)
    end)
  end
end
