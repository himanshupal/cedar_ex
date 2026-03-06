defmodule CedarPolicy.EntityTypeName do
  defstruct [:namespace, :basename]

  @type t :: %CedarPolicy.EntityTypeName{basename: String.t(), namespace: String.t() | nil}
  @spec new(basename :: atom() | String.t(), namespace :: atom() | String.t() | nil) :: t()

  def new(basename, namespace \\ nil)

  def new(basename, namespace) when is_atom(basename) and is_nil(namespace) do
    new(to_string(basename), namespace)
  end

  def new(basename, namespace) when is_binary(basename) and is_nil(namespace) do
    %CedarPolicy.EntityTypeName{namespace: namespace, basename: basename}
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
    %CedarPolicy.EntityTypeName{namespace: namespace, basename: basename}
  end
end
