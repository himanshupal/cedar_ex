defmodule CedarPolicy.Record do
  @moduledoc """

  """

  alias CedarPolicy.Entity

  @datetime_regex ~r"^\d{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])(?:T(?:[01]\d|2[0-3]):(?:[0-5]\d):(?:[0-5]\d)(?:\.\d{3})?(?:Z|[+-](?:[01]\d|2[0-3])(?:[0-5]\d)))?$"
  @duration_regex ~r"^(\d+d){0,1}(\d+h){0,1}(\d+m){0,1}(\d+s){0,1}(\d+ms){0,1}$"

  @type restricted_expression ::
          {:set, list(t())}
          | {:long, integer()}
          | {:bool, boolean()}
          | {:ip, String.t()}
          | {:string, String.t()}
          | {:decimal, String.t()}
          | {:date_time, String.t()}
          | {:duration, String.t()}
          | {:entity, Entity.t()}
          | {:record, list({String.t(), t()})}

  @type t :: list({String.t(), restricted_expression})

  @type value() ::
          atom() | integer() | float() | boolean() | String.t() | Entity.t() | DateTime.t() | t()

  @spec new(
          data ::
            list({atom() | String.t(), value() | list(value())})
        ) :: t()
  def new(data) do
    Enum.map(data, fn {key, value} ->
      {parse_key(key), parse_value(value)}
    end)
  end

  defp parse_key(key) when is_binary(key), do: key

  defp parse_key(key) when is_atom(key), do: to_string(key)

  defp parse_value(value) when is_boolean(value), do: {:bool, value}

  defp parse_value(value) when is_integer(value), do: {:long, value}

  defp parse_value(value) when is_float(value), do: {:decimal, to_string(value)}

  defp parse_value(value) when is_atom(value) or is_binary(value) do
    cond do
      is_atom(value) ->
        {:string, to_string(value)}

      String.match?(value, @datetime_regex) ->
        {:date_time, value}

      String.match?(value, @duration_regex) ->
        {:duration, value}

      is_ip?(value) ->
        {:ip, value}

      true ->
        {:string, value}
    end
  end

  defp parse_value(value) when is_struct(value, Entity), do: {:entity, value}

  defp parse_value(value) when is_struct(value, DateTime),
    do: {:date_time, DateTime.to_iso8601(value)}

  defp parse_value(value) when is_list(value) do
    cond do
      Enum.all?(value, &is_tuple/1) -> {:record, value}
      true -> {:set, Enum.map(value, &parse_value/1)}
    end
  end

  defp is_ip?(value) do
    if String.contains?(value, "/") do
      [addr, prefix] = String.split(value, "/", parts: 2)
      has_valid_suffix = match?({p, ""} when p in 0..128, Integer.parse(prefix))
      is_ip?(addr) and has_valid_suffix
    else
      match?({:ok, _}, :inet.parse_address(String.to_charlist(value)))
    end
  end
end
