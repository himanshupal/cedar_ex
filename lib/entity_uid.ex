defmodule CedarPolicy.EntityUid do
  @moduledoc """
  Represents a unique identifier for an entity. Consists of an entity type name and an ID.
  """
  defstruct [:type_name, :id]

  alias CedarPolicy.EntityTypeName

  @type t() :: %__MODULE__{type_name: String.t(), id: String.t()}

  @valid_entity_type_name_regex [
    {:basename, ~r"^[a-zA-Z]+$"},
    {:entity_type_name, ~r"^([a-zA-Z]+:{2})+[a-zA-Z]+$"}
  ]

  @doc """
  Creates a new `CedarPolicy.EntityUid` struct.

  ## Parameters
    - `type_name`: The entity type name, which can be a `CedarPolicy.EntityTypeName` struct or a string.
    - `id`: The unique identifier for the entity, which can be a string or an atom.

  ## Examples

      iex> EntityUid.new(:user, :123)
      %EntityUid{type_name: "user", id: "123"}

      iex> EntityUid.new("Admin::user", :123)
      %EntityUid{type_name: "Admin::user", id: "123"}

      iex> EntityUid.new(EntityTypeName.new(basename: "user", namespace: "Admin"), "456")
      %EntityUid{type_name: "Admin::user", id: "456"}
  """
  @spec new(type_name :: EntityTypeName.t() | String.t(), id :: String.t() | atom()) :: t()

  def new(type_name, id) when is_atom(id) do
    new(type_name, to_string(id))
  end

  def new(%EntityTypeName{namespace: nil} = type_name, id)
      when is_struct(type_name, EntityTypeName) and is_binary(id) do
    %__MODULE__{type_name: type_name.basename, id: id}
  end

  def new(type_name, id) when is_struct(type_name, EntityTypeName) and is_binary(id) do
    %__MODULE__{type_name: "#{type_name.namespace}::#{type_name.basename}", id: id}
  end

  def new(type_name, id) when is_binary(type_name) and is_binary(id) do
    type_name |> validate |> from_string(id)
  end

  defp from_string({:basename, basename}, id) do
    entity_type_name = EntityTypeName.new(basename)
    new(entity_type_name, id)
  end

  defp from_string({:entity_type_name, entity_type_name}, id) do
    %__MODULE__{type_name: entity_type_name, id: id}
  end

  defp validate(value) do
    {type_name, _} =
      Enum.find(@valid_entity_type_name_regex, {:invalid_entity_type_name, nil}, fn {_, regex} ->
        String.match?(value, regex)
      end)

    {type_name, value}
  end
end
