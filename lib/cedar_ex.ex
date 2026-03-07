defmodule CedarPolicy do
  alias CedarPolicy.Native

  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  @spec get_lang_version() :: CedarPolicy.Version.t()
  def get_lang_version(), do: Native.get_lang_version()

  @spec get_sdk_version() :: CedarPolicy.Version.t()
  def get_sdk_version(), do: Native.get_sdk_version()

  def create_request(state, p, a, r, c \\ [], s \\ nil)
  def create_request(state, p, a, r, c, s), do: Native.create_request(state, p, a, r, c, s)
end
