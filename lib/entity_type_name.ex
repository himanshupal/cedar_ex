defmodule CedarPolicy.EntityTypeName do
  @moduledoc """
  Represents an entity type name. Consists of a namespace and the type name.
  """
  defstruct [:namespace, :basename]

  @type t :: %__MODULE__{basename: String.t(), namespace: String.t() | nil}

  @doc """
  Creates a new `CedarPolicy.EntityTypeName` struct.

  ## Parameters
    - `basename`: The base name of the entity type.
    - `namespace`: The namespace of the entity type (optional).

  ## Examples

      iex> EntityTypeName.new(:user)
      %EntityTypeName{basename: "user", namespace: nil}

      iex> EntityTypeName.new(:user, :admin)
      %EntityTypeName{basename: "user", namespace: "admin"}

  """
  @spec new(basename :: atom() | String.t(), namespace :: atom() | String.t() | nil) :: t()

  def new(basename, namespace \\ nil)

  def new(basename, namespace) when is_atom(basename) and is_nil(namespace) do
    new(to_string(basename), namespace)
  end

  def new(basename, namespace) when is_binary(basename) and is_nil(namespace) do
    %__MODULE__{namespace: namespace, basename: basename}
  end

  def new(basename, namespace) when is_atom(basename) and is_atom(namespace) do
    new(to_string(basename), to_string(namespace))
  end

  def new(basename, namespace) when is_atom(basename) and is_binary(namespace) do
    new(to_string(basename), namespace)
  end

  def new(basename, namespace) when is_binary(basename) and is_atom(namespace) do
    new(basename, to_string(namespace))
  end

  def new(basename, namespace) when is_binary(basename) and is_binary(namespace) do
    %__MODULE__{namespace: namespace, basename: basename}
  end
end
