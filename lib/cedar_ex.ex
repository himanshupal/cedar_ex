defmodule CedarPolicy do
  alias CedarPolicy.Native

  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  def new do
    Native.new_context()
  end

  def increment(context) do
    Native.increment(context)
  end
end
