defmodule CedarPolicy do
  alias CedarPolicy.Native

  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  def new(policy) do
    Native.new_context(policy)
  end

  @spec get_lang_version() :: CedarPolicy.Native.Version.t()
  def get_lang_version(), do: Native.get_lang_version()

  def get_sdk_version(), do: Native.get_sdk_version()

  def create_entity_uid(entity_name, id), do: Native.create_entity_uid(entity_name, id)
  def create_request(p, a, r), do: Native.create_request(p, a, r)

  def get_policy_as_json(context), do: Native.get_policy_as_json(context)
end
