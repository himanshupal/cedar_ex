defmodule CedarPolicy.Native do
  use Rustler, otp_app: :cedar_policy, crate: "cedar_ex"

  def new_context(_policy), do: not_loaded()

  @spec get_lang_version() :: CedarPolicy.Version.t()
  def get_lang_version(), do: not_loaded()

  @spec get_sdk_version() :: CedarPolicy.Version.t()
  def get_sdk_version(), do: not_loaded()

  def get_policy_as_json(_context), do: not_loaded()

  def create_request(_p, _a, _r, _c, _s), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
