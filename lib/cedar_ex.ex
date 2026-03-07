defmodule CedarPolicy do
  alias CedarPolicy.Native

  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  def new(policy) do
    Native.new_context(policy)
  end

  @spec get_lang_version() :: CedarPolicy.Version.t()
  def get_lang_version(), do: Native.get_lang_version()

  @spec get_sdk_version() :: CedarPolicy.Version.t()
  def get_sdk_version(), do: Native.get_sdk_version()

  def create_request(p, a, r, c \\ [], s \\ nil)
  def create_request(p, a, r, c, s), do: Native.create_request(p, a, r, c, s)

  def get_policy_as_json(context), do: Native.get_policy_as_json(context)
end
