defmodule CedarPolicy do
  alias CedarPolicy.EntityUid
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

  @spec create_request(
          principal :: EntityUid.t(),
          action :: EntityUid.t(),
          resource :: EntityUid.t()
        ) :: nil
  def create_request(p, a, r), do: Native.create_request(p, a, r)

  def get_policy_as_json(context), do: Native.get_policy_as_json(context)
end
