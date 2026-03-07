defmodule CedarPolicy.Native do
  use Rustler, otp_app: :cedar_policy, crate: "cedar_ex"

  def new(), do: not_loaded()

  def add_policy(_state, _policy, _id \\ nil)
  def add_policy(_state, _policy, _id), do: not_loaded()

  def add_entities(_state, _entities, _schema \\ nil)
  def add_entities(_state, _entities, _schema), do: not_loaded()

  def validate(_state, _schema), do: not_loaded()

  @spec get_lang_version() :: CedarPolicy.Version.t()
  def get_lang_version(), do: not_loaded()

  @spec get_sdk_version() :: CedarPolicy.Version.t()
  def get_sdk_version(), do: not_loaded()

  def create_request(_state, _p, _a, _r, _c, _s), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
