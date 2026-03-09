defmodule CedarPolicy do
  alias CedarPolicy.Native

  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  @spec get_lang_version() :: Version.t()
  def get_lang_version(), do: Native.get_lang_version()

  @spec get_sdk_version() :: Version.t()
  def get_sdk_version(), do: Native.get_sdk_version()
end
