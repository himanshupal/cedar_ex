defmodule CedarPolicy.EntityUid do
  defstruct [:type_name, :id]

  alias CedarPolicy.EntityTypeName

  @type t() :: %CedarPolicy.EntityUid{type_name: String.t(), id: String.t()}
  @spec new(type_name :: EntityTypeName.t() | String.t(), id :: String.t() | atom()) :: t()

  @valid_entity_type_name_regex [
    {:basename, ~r"^[a-zA-Z]+$"},
    {:entity_type_name, ~r"^([a-zA-Z]+:{2})+[a-zA-Z]+$"}
  ]

  def new(type_name, id) when is_atom(id) do
    new(type_name, to_string(id))
  end

  def new(%EntityTypeName{namespace: nil} = type_name, id)
      when is_struct(type_name, EntityTypeName) and is_binary(id) do
    %CedarPolicy.EntityUid{type_name: type_name.basename, id: id}
  end

  def new(type_name, id) when is_struct(type_name, EntityTypeName) and is_binary(id) do
    %CedarPolicy.EntityUid{type_name: "#{type_name.namespace}::#{type_name.basename}", id: id}
  end

  def new(type_name, id) when is_binary(type_name) and is_binary(id) do
    type_name |> validate |> from_string(id)
  end

  defp from_string({:basename, basename}, id) do
    entity_type_name = EntityTypeName.new(basename)
    new(entity_type_name, id)
  end

  defp from_string({:entity_type_name, entity_type_name}, id) do
    %CedarPolicy.EntityUid{type_name: entity_type_name, id: id}
  end

  defp validate(value) do
    {type_name, _} =
      Enum.find(@valid_entity_type_name_regex, {:invalid_entity_type_name, nil}, fn {_, regex} ->
        String.match?(value, regex)
      end)

    {type_name, value}
  end
end
